use std::ops::Deref;

use anyhow::Result;
use qdrant_client::{
    config::QdrantConfig,
    qdrant::{
        sparse_vectors_config, CollectionExistsRequest, CreateCollectionBuilder, Distance, Fusion, PrefetchQueryBuilder, Query, QueryPointsBuilder, ScoredPoint, SearchPoints, SearchPointsBuilder, SearchResponse, SparseVectorConfig, SparseVectorParams, SparseVectorParamsBuilder, SparseVectorsConfigBuilder, VectorParamsBuilder, VectorsConfigBuilder
    },
    Qdrant, QdrantError,
};

pub struct MQdrantClient {
    qdrant: Qdrant,
    all_collections: Vec<String>,
}

impl Deref for MQdrantClient {
    type Target = Qdrant;

    fn deref(&self) -> &Self::Target {
        &self.qdrant
    }
}

impl MQdrantClient {
    pub fn new() -> Result<MQdrantClient> {
        let endpoint = dotenv::var("QDRANT_ENDPOINT").unwrap();
        let qdrant = QdrantConfig::from_url(&endpoint).build().unwrap();
        Ok(Self {
            qdrant,
            all_collections: vec![],
        })
    }

    pub async fn create_default_collection(&mut self, collection_name: String) -> Result<()> {
        let vector_size = 768;
        let vec_params = VectorParamsBuilder::new(vector_size, Distance::Cosine).build();
        let vectors_config = VectorsConfigBuilder::default()
            .add_vector_params(vec_params)
            .clone();
        match self
            .qdrant
            .create_collection(
                CreateCollectionBuilder::new(&collection_name).vectors_config(vectors_config),
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.into()),
        };
        self.all_collections.push(collection_name);
        Ok(())
    }

    pub async fn create_named_vectors_collection(&mut self, collection_name: String) -> Result<()> {
        let dense_params = VectorParamsBuilder::new(768, Distance::Cosine).build();
        let sparse_params = SparseVectorParams::default();
        let mut vectors_config = VectorsConfigBuilder::default();
        vectors_config.add_named_vector_params("dense", dense_params);
        let mut sparse_vector_config = SparseVectorsConfigBuilder::default();
        sparse_vector_config.add_named_vector_params("sparse", sparse_params);

        match self
            .qdrant
            .create_collection(
                CreateCollectionBuilder::new(&collection_name)
                    .vectors_config(vectors_config)
                    .sparse_vectors_config(sparse_vector_config),
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.into()),
        };

        self.all_collections.push(collection_name);
        Ok(())
    }

    pub async fn init_qdrant_collection(&mut self, collection_name: &str) -> Result<()> {
        match self
            .collection_exists(CollectionExistsRequest {
                collection_name: collection_name.to_string(),
            })
            .await
            .unwrap()
        {
            true => Ok(()),
            false => {
                self.create_named_vectors_collection(collection_name.to_string())
                    .await
                    .unwrap();
                Ok(())
            }
        }
    }

    pub async fn search_query(
        &self,
        collection_name: &str,
        embeddings: Vec<f32>,
        indices: Option<Vec<u32>>,
    ) -> Result<SearchResponse, QdrantError> {
        match indices {
            Some(indices) => {
                self.search_points(
                    SearchPointsBuilder::new(collection_name, embeddings, 10)
                        .sparse_indices(indices)
                        .vector_name("sparse")
                        .with_payload(true)
                        .build(),
                )
                .await
            }
            None => {
                self.search_points(
                    SearchPointsBuilder::new(collection_name, embeddings, 10)
                        .vector_name("dense")
                        .with_payload(true),
                )
                .await
            }
        }
    }

    pub async fn hybrid_search(
        &self,
        collection_name: &str,
        dense_embeddings: Vec<f32>,
        sparse_embeddings: Vec<f32>,
        sparse_indices: Vec<u32>,
    ) -> Vec<ScoredPoint> {
        let sparse= sparse_indices.into_iter().zip(sparse_embeddings).collect::<Vec<_>>();
        let query = QueryPointsBuilder::new(collection_name)
            .add_prefetch(PrefetchQueryBuilder::default()
                .query(Query::new_nearest(sparse.as_slice())).using("sparse").limit(20u64))
            .add_prefetch(PrefetchQueryBuilder::default()
                .query(Query::new_nearest(dense_embeddings)).using("dense").limit(20u64))
            .with_payload(true)
            .query(Query::new_fusion(Fusion::Rrf));
        
        self.query(query).await.unwrap().result
        
        // TODO
    }
}
