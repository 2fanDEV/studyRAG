use std::{cell::RefCell, marker::PhantomData, sync::Arc, thread};

use actix::{Addr, SyncArbiter};
use actix_web::HttpResponse;
use anyhow::{anyhow, Result};
use bson::{doc, Uuid};
use log::debug;
use lopdf::Document;
use mongodb::Collection;
use qdrant_client::{
    qdrant::{PointId, PointStruct, QueryPoints, QueryPointsBuilder, UpsertPointsBuilder},
    Payload,
};
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde_json::json;

use crate::collection_values::media::Media;
use crate::{
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    embedding::{
        bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
            EmbeddingActor, ExtractionActor,
        },
        processer::TextProcessor,
        EmbeddingMessage, BertRequest,
    },
    endpoints::query::QueryRequest,
};

const DOCUMENT_EMBEDDINGS: &'static str = "document_embeddings";


#[allow(unused)]
pub struct EmbeddableService {
    qdrant: RefCell<MQdrantClient>,
    text_processor: TextProcessor,
    vector_embedding_actor: Addr<EmbeddingActor>,
    keyword_actor: Addr<ExtractionActor>,
    media_collection: Collection<Media>,
    keywords_collection: Collection<Keyword>,
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
            keywords_collection: mongo_client.database("RAG").collection("vocabulary"),
            vector_embedding_actor: embedding_actor,
            text_processor: TextProcessor::new(chunk_size).unwrap(),
            keyword_actor: extracting_actor,
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

        let (point_embeddings, keywords) = self
            .create_embedded_points::<PointStruct>(text.clone(), |embedding, keywords| {
                PointStruct::new(
                    PointId::from(Uuid::new().to_string()),
                    embedding,
                    Payload::try_from(json!({
                        "mongo_db_id":  mongo_db_id,
                        "text": text,
                    }))
                    .unwrap(),
                )
            })
            .await
            .unwrap();

        let res = match mut_qdrant
            .upsert_points(UpsertPointsBuilder::new(
                DOCUMENT_EMBEDDINGS,
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
        let (query_embeddings, keywords) = match self
            .create_embedded_points::<QueryPoints>(query.text, |embedding, keywords| {
                QueryPointsBuilder::new(DOCUMENT_EMBEDDINGS)
                    .with_payload(true)
                    .query(embedding)
                    .build()
            })
            .await
        {
            Ok(points) => points,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        // right now its only a single query from the frontend in form of a prompt so we take first element
        let query = if let Some(points) = query_embeddings.get(0).take() {
            let query = self.qdrant.borrow().query(points.clone()).await.unwrap();
            query
                .result
                .iter()
                .map(|point| ((point.payload.clone(), point.score)))
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        HttpResponse::Ok().json(query)
    }

    async fn keyword_extraction(&self, text: &str) -> Vec<Vec<Keyword>> {
        let keywords = self
            .keyword_actor
            .send(BertRequest {
                text: vec![EmbeddingMessage {
                    text: text.to_string(),
                }],
                _data: PhantomData,
            })
            .await
            .unwrap();
        debug!("{:?}", keywords);
        keywords
    }

    async fn create_embedded_points<T>(
        &self,
        text: String,
        create_point: impl Fn(Vec<f32>, Vec<Keyword>) -> T,
    ) -> Result<(Vec<T>, Vec<Keyword>)> {
        let mut point_embeddings: Vec<T> = vec![];
        let mut keywords = vec![];
        let mut handles = vec![];
        let chunked_text = self.text_processor.process(&text)?;
            
        handles.push(tokio::spawn(|| self.keyword_actor.send(EmbeddingMessage {
            text: chunked_text
        })));
        
        handles.push(tokio::spawn(todo!()));
        
        handles.iter().for_each(|hndl| {

        });

       /* for (_index, text) in chunked_text.into_iter().enumerate() {
            keywords.push(
                self.keyword_extraction(&text)
                    .await
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Keyword>>(),
            );
            let embedding = self
                .vector_embedding_actor
                .send(BertRequest {
                    text: vec![BertMessage { text }],
                    _data: PhantomData,
                })
                .await
                .unwrap()[0]
                .clone();
            let embedded_point = create_point(
                embedding,
                keywords.clone().into_iter().flatten().collect::<Vec<_>>(),
            );
            point_embeddings.push(embedded_point);
        } */
        let keywords = keywords.into_iter().flatten().collect::<Vec<_>>();
        Ok((point_embeddings, keywords))
    }
}
