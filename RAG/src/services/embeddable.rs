use std::sync::Arc;

use actix::{Addr, SyncArbiter};
use actix_web::{web::Payload, HttpResponse};

use crate::{
    database::qdrant::MQdrantClient,
    model::bert_actors::{
            EmbeddingActor, ExtractionActor,
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
        },
};

pub struct EmbeddableService {
    qdrant: Arc<MQdrantClient>,
    vector_embedding_actor: Addr<EmbeddingActor>,
    keyword_actor: Addr<ExtractionActor>,
}

impl EmbeddableService {
    pub fn new(
        qdrant: Arc<MQdrantClient>,
    ) -> Self {
        let embedding_actor = SyncArbiter::start(1, || {
            EmbeddingActor::new(Box::new(VectorEmbeddingModel::new().unwrap()))
        });
        let extracting_actor = SyncArbiter::start(1, || {
            ExtractionActor::new(Box::new(KeywordExtractionModel::new().unwrap()))
        });
        Self {
            qdrant,
            vector_embedding_actor: embedding_actor,
            keyword_actor: extracting_actor,
        }
    }

    pub async fn upload(&self, payload: Payload) -> HttpResponse {
        //TODO
        todo!()        
    }
        

    fn calculate_embeddings_from_text(&self) {}
}
