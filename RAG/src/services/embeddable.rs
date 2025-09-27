use std::{cell::RefCell, collections::HashMap, marker::PhantomData, sync::Arc};

use actix::{Addr, SyncArbiter};
use actix_web::HttpResponse;
use anyhow::{anyhow, Result};
use bson::uuid;
use log::debug;
use lopdf::Document;
use mongodb::Collection;
use qdrant_client::{
    qdrant::{
        CollectionExistsRequest, PointId, PointStruct, QueryPointsBuilder, UpsertPointsBuilder,
        Value,
    },
    Payload,
};
use serde_json::json;
use tokenizers::Result;

use crate::{
    collection_values::media::{Media, MediaInformation},
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    embedding::{
        bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
            EmbeddingActor, ExtractionActor,
        }, processer::{TextProcessor, TokenAndText}, BertMessage, BertRequest, Message
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

        let pages = document
            .get_pages()
            .iter()
            .map(|page| *page.0)
            .collect::<Vec<_>>();
        let text = document.extract_text(&pages).unwrap();
        let point_embeddings = self.create_embedded_text(mongo_db_id, text).await.unwrap();
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
        let query_embeddings = self
            .vector_embedding_actor
            .send(BertRequest {
                text: vec![BertMessage {
                    text: query.text,
                }],
                _data: PhantomData,
            })
            .await
            .unwrap();

        debug!("VECTOR={:?}", query_embeddings);
        let query1 = self
            .qdrant
            .borrow()
            .query(
                QueryPointsBuilder::new(DOCUMENT_EMBEDDINGS)
                    .with_payload(true)
                    .query(query_embeddings[0].clone()),
            )
            .await
            .unwrap();
        debug!("{:?}", query1);
        let x = query1
            .result
            .iter()
            .map(|point| (point.payload.clone()))
            .collect::<Vec<_>>();

        HttpResponse::Ok().json(x)
    }

    async fn create_embedded_text(
        &self,
        mongo_db_id: String,
        text: String,
    ) -> tokenizers::Result<Vec<PointStruct>> {
        let mut point_embeddings = vec![];
        let token_and_texts: Vec<TokenAndText> = self.text_processor.process(&text)?;
               
        for (index, (text, tokens)) in token_and_texts.into_iter().enumerate() {

            point_embeddings.push(PointStruct::new(
                PointId::from(uuid::Uuid::new().to_string()),
                tokens,
                Payload::try_from(json!({"mongo_db_id": mongo_db_id})).unwrap(),
            ));
        }
        point_embeddings */
        vec![]
    }
}
