use std::{cell::RefCell, collections::HashMap, sync::Arc};

use actix::{Addr, SyncArbiter};
use actix_web::HttpResponse;
use anyhow::{anyhow, Result};
use bson::uuid;
use lopdf::Document;
use mongodb::Collection;
use qdrant_client::{
    qdrant::{
        CollectionExistsRequest, PointId, PointStruct,
        UpsertPointsBuilder, Value
    },
    Payload,
};
use serde_json::json;

use crate::{
    collection_values::media::Media,
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    model::{
        bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
            EmbeddingActor, ExtractionActor,
        },
        split_by_context_size,
    },
};

pub struct EmbeddableService {
    qdrant: RefCell<MQdrantClient>,
    vector_embedding_actor: Addr<EmbeddingActor>,
    media_collection: Collection<Media>,
    keyword_actor: Addr<ExtractionActor>,
}

impl EmbeddableService {
    pub fn new(qdrant: RefCell<MQdrantClient>, mongo_client: Arc<MongoClient>) -> Self {
        let embedding_actor = SyncArbiter::start(1, || {
            EmbeddingActor::new(Box::new(VectorEmbeddingModel::new().unwrap()))
        });
        let extracting_actor = SyncArbiter::start(1, || {
            ExtractionActor::new(Box::new(KeywordExtractionModel::new().unwrap()))
        });

        Self {
            qdrant,
            media_collection: mongo_client.database("RAG").collection("media"),
            vector_embedding_actor: embedding_actor,
            keyword_actor: extracting_actor,
        }
    }

    pub async fn upload(&self, mongo_db_id: String, buf: Vec<u8>) -> Result<HttpResponse> {
        let document = match Document::load_mem(&buf) {
            Ok(doc) => doc,
            Err(err) => return Err(anyhow!(err.to_string())),
        };
        let mut mut_qdrant = self.qdrant.borrow_mut();
        match mut_qdrant
            .collection_exists(CollectionExistsRequest {
                collection_name: "document_embeddings".to_string(),
            })
            .await
            .unwrap()
        {
            true => {}
            false => {
                mut_qdrant
                    .create_default_collection("document_embeddings".to_string())
                    .await
                    .unwrap();
            }
        };

        let pages = document
            .get_pages()
            .iter()
            .map(|page| *page.0)
            .collect::<Vec<_>>();
        let text = document.extract_text(&pages).unwrap();
        let point_embeddings = self.calculate_embeddings_from_text(mongo_db_id, text).await;
        let res = match mut_qdrant
            .upsert_points(UpsertPointsBuilder::new(
                "document_embeddings",
                point_embeddings,
            ))
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.time.to_string()),
            Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        };
        Ok(res)
    }

    async fn calculate_embeddings_from_text(&self, mongo_db_id: String, text: String) -> Vec<PointStruct> {
        let formatted_text = split_by_context_size::<Vec<Vec<f32>>>(text, 100, 8);
        let embeddings = self
            .vector_embedding_actor
            .send(formatted_text)
            .await
            .unwrap();

        let mut point_embeddings = vec![];
        let hashmap: HashMap<String, Value> = HashMap::new();
        for (index, vector) in embeddings.into_iter().enumerate() {
            point_embeddings.push(PointStruct::new(
                PointId::from(uuid::Uuid::new().to_string()),
                vector,
                Payload::from(hashmap.clone())
            ));
        }
        point_embeddings
    }
}
