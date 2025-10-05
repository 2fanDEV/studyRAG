use std::ops::Deref;

use anyhow::Result;
use qdrant_client::{
    config::QdrantConfig,
    qdrant::{
        vectors_config_diff::Config, CollectionExistsRequest, CreateCollection,
        CreateCollectionBuilder, Distance, VectorParams, VectorParamsBuilder, VectorsConfigBuilder,
    },
    Qdrant,
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
                self.create_default_collection(collection_name.to_string())
                    .await
                    .unwrap();
                Ok(())
            }
        }
    }
}
