use std::{cell::RefCell, marker::PhantomData, sync::Arc};

use actix::{Addr, SyncArbiter};
use actix_web::{HttpRequest, HttpResponse};
use anyhow::{anyhow, Result};
use bson::uuid;
use log::debug;
use lopdf::Document;
use mongodb::Collection;
use qdrant_client::{
    qdrant::{
        CollectionExistsRequest, PointId, PointStruct, QueryPoints, QueryPointsBuilder,
        UpsertPointsBuilder,
    },
    Payload,
};
use serde_json::json;

use crate::{
    collection_values::media::{Media, MediaInformation},
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    embedding::{
        bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
            EmbeddingActor, ExtractionActor,
        },
        processer::TextProcessor,
        BertMessage, BertRequest,
    },
    endpoints::query::QueryRequest,
};

const DOCUMENT_EMBEDDINGS: &'static str = "document_embeddings";

pub struct EmbeddableService {
    qdrant: RefCell<MQdrantClient>,
    text_processor: TextProcessor,
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
        let chunk_size = 512;
        Self {
            qdrant,
            media_collection: mongo_client.database("RAG").collection("media"),
            vector_embedding_actor: embedding_actor,
            text_processor: TextProcessor::new(chunk_size).unwrap(),
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
                    .create_default_collection(DOCUMENT_EMBEDDINGS.to_string())
                    .await
                    .unwrap();
            }
        };

        let pages = document.get_pages().into_iter().map(|n| n.0).collect::<Vec<_>>();
        let text = document.extract_text(&pages).unwrap().replace("\n", " ");
        let point_embeddings = self
            .create_embedded_point_structs(mongo_db_id, text)
            .await
            .unwrap();
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

    pub async fn execute_query(&self, query: QueryRequest) -> HttpResponse {
        let query_points = self.create_query_points_embedded(query.text).await.unwrap()[0].clone();
        let query1 = self.qdrant.borrow().query(query_points).await.unwrap();

        let x = query1
            .result
            .iter()
            .map(|point| ((point.payload.clone(), point.score)))
            .collect::<Vec<_>>();

        HttpResponse::Ok().json(x)
    }

    async fn create_query_points_embedded(
        &self,
        text: String,
    ) -> tokenizers::Result<Vec<QueryPoints>> {
        let mut point_embeddings: Vec<QueryPoints> = vec![];
        let chunked_text = self.text_processor.process(&text)?;
        for (index, text) in chunked_text.into_iter().enumerate() {
            let embedding = self
                .vector_embedding_actor
                .send(BertRequest {
                    text: vec![BertMessage { text }],
                    _data: PhantomData,
                })
                .await
                .unwrap()[0]
                .clone();
            let query_points = QueryPointsBuilder::new(DOCUMENT_EMBEDDINGS)
                .with_payload(true)
                .query(embedding)
                .build();
            debug!("query_points={:?}", query_points);
            point_embeddings.push(query_points);
        }
        Ok(point_embeddings)
    }

    async fn create_embedded_point_structs(
        &self,
        mongo_db_id: String,
        text: String,
    ) -> tokenizers::Result<Vec<PointStruct>> {
        let mut point_embeddings = vec![];
        let token_and_texts = self.text_processor.process(&text)?;

        for (index, text) in token_and_texts.into_iter().enumerate() {
            let embedding = self
                .vector_embedding_actor
                .send(BertRequest {
                    text: vec![BertMessage { text: text.clone() }],
                    _data: PhantomData,
                })
                .await
                .unwrap()[0]
                .clone();
            point_embeddings.push(PointStruct::new(
                PointId::from(uuid::Uuid::new().to_string()),
                embedding,
                Payload::try_from(json!({
                    "mongo_db_id": mongo_db_id,
                    "text_passage": text}))
                .unwrap(),
            ));
        }
        debug!("POINTS={point_embeddings:?}");
        Ok(point_embeddings)
    }
}
