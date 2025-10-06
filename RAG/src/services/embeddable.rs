use std::{
    cell::RefCell, collections::HashMap, iter::zip, marker::PhantomData, sync::Arc, thread,
    time::SystemTime,
};

use actix::{Addr, SyncArbiter};
use actix_web::HttpResponse;
use anyhow::{anyhow, Result};
use bson::Uuid;
use log::debug;
use lopdf::Document;
use mongodb::Collection;
use qdrant_client::{
    qdrant::{
        DenseVector, NamedVectors, PointId, PointStruct, QueryPoints, QueryPointsBuilder,
        SearchPoints, SparseVector, UpsertPointsBuilder, Vector,
    },
    Payload,
};
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde_json::json;

use crate::{
    collection_values::media::Media,
    embedding::{
        embedding_actors::{EmbeddingActor, SpecifiedModelType},
        sparse::{self, SparseTextEmbeddingModel},
    },
};
use crate::{
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    embedding::{processer::TextProcessor, BertRequest, EmbeddingMessage},
    endpoints::query::QueryRequest,
};

const DOCUMENT_EMBEDDINGS: &'static str = "document_embeddings";

#[allow(unused)]
pub struct EmbeddableService {
    qdrant: RefCell<MQdrantClient>,
    text_processor: TextProcessor,
    dense_embedding_actor: Addr<EmbeddingActor>,
    sparse_embedding_actor: SparseTextEmbeddingModel,
    media_collection: Collection<Media>,
}

impl EmbeddableService {
    pub fn new(qdrant: RefCell<MQdrantClient>, mongo_client: Arc<MongoClient>) -> Self {
        let dense_embedding_actor = SyncArbiter::start(1, || {
            EmbeddingActor::new(SpecifiedModelType::DENSE(rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType::SentenceT5Base))
        });

        let sparse_embedding_actor =
            SparseTextEmbeddingModel::new(Some("prithivida/Splade_PP_en_v2"));

        let chunk_size = 512;
        Self {
            qdrant,
            media_collection: mongo_client.database("RAG").collection("media"),
            dense_embedding_actor,
            sparse_embedding_actor,
            text_processor: TextProcessor::new(chunk_size).unwrap(),
        }
    }

    pub async fn upload(&self, mongo_db_id: String, buf: Vec<u8>) -> Result<HttpResponse> {
        let mut mut_qdrant = self.qdrant.borrow_mut();
        match mut_qdrant.init_qdrant_collection(DOCUMENT_EMBEDDINGS).await {
            Ok(_) => {}
            Err(_) => return Err(anyhow!("Failed to initialize collection")),
        }
        let document = match Document::load_mem(&buf) {
            Ok(doc) => doc,
            Err(err) => return Err(anyhow!(err.to_string())),
        };
        let pages = document
            .get_pages()
            .into_iter()
            .map(|n| n.0)
            .collect::<Vec<_>>();
        let text = document.extract_text(&pages).unwrap().replace("\n", " ");

        let point_embeddings = self
            .create_embedded_points::<PointStruct>(
                text.clone(),
                |dense_embedding, sparse_embedding, text_chunk| {
                    let sparse_vec = SparseVector {
                        indices: sparse_embedding.0,
                        values: sparse_embedding.1,
                    };

                    let dense_vec = DenseVector {
                        data: dense_embedding,
                    };

                    PointStruct::new(
                        PointId::from(Uuid::new().to_string()),
                        NamedVectors {
                            vectors: HashMap::<String, Vector>::from([
                                ("dense".to_string(), Vector::from(dense_vec)),
                                ("sparse".to_string(), Vector::from(sparse_vec)),
                            ]),
                        },
                        Payload::try_from(json!({
                            "mongo_db_id":  mongo_db_id,
                            "text": text_chunk,
                        }))
                        .unwrap(),
                    )
                },
            )
            .await
            .unwrap();

        debug!("{:?}", point_embeddings);
        let res = match mut_qdrant
            .upsert_points(UpsertPointsBuilder::new(
                DOCUMENT_EMBEDDINGS,
                point_embeddings,
            ))
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.time.to_string()),
            Err(err) => {
                debug!("{:?}", err);
                HttpResponse::InternalServerError().json(err.to_string())
            }
        };
        Ok(res)
    }

    pub async fn execute_query(&self, query: QueryRequest) -> HttpResponse {
        let query_embeddings = match self
            .create_embedded_points::<(Vec<f32>, (Vec<u32>, Vec<f32>))>(
                query.text,
                |dense, sparse, text_chunk| (dense, sparse),
            )
            .await
        {
            Ok(points) => points,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        // right now its only a single query from the frontend in form of a prompt so we take first element
        let scored_points = if let Some(points) = query_embeddings.get(0).take() {
            let dense_points = points.0.clone();
            let sparse_indices = points.1 .0.clone();
            let sparse_points = points.1 .1.clone();
            self.qdrant
                .borrow()
                .hybrid_search(
                    DOCUMENT_EMBEDDINGS,
                    dense_points,
                    sparse_points,
                    sparse_indices,
                )
                .await
                .iter().map(|rs| {
                    (rs.score, rs.payload.clone())
                })
                .collect::<_>()
        } else {
            vec![]
        };

        HttpResponse::Ok().json(scored_points)
    }

    async fn create_embedded_points<T>(
        &self,
        text: String,
        create_point: impl Fn(Vec<f32>, (Vec<u32>, Vec<f32>), String) -> T,
    ) -> Result<Vec<T>> {
        let start = SystemTime::now();
        let mut point_embeddings: Vec<T> = vec![];
        let chunked_text = self.text_processor.process(&text)?;

        let dense_embeddings = self
            .dense_embedding_actor
            .send(BertRequest {
                text: EmbeddingMessage {
                    text: chunked_text.clone(),
                },
                _data: PhantomData,
            })
            .await
            .unwrap();

        let sparse_embedding = self
            .sparse_embedding_actor
            .process(BertRequest {
                text: EmbeddingMessage {
                    text: chunked_text.clone(),
                },
                _data: PhantomData,
            })
            .await
            .into_iter()
            .map(|embedding| {
                embedding
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, emb)| *emb != 0.0)
                    .map(|(idx, val)| (idx as u32, val as f32))
                    .unzip()
            })
            .collect::<Vec<(Vec<u32>, Vec<f32>)>>();

        zip(dense_embeddings, sparse_embedding)
            .zip(chunked_text)
            .into_iter()
            .for_each(|((dense_emb, sparse_emb), text)| {
                point_embeddings.push(create_point(dense_emb, sparse_emb, text));
            });

        let time_elapsed = start.elapsed().unwrap();
        debug!(
            "Dense and Sparse Embedding Time Duration: {:?}/s",
            time_elapsed.as_secs_f64()
        );

        Ok(point_embeddings)
    }
}
