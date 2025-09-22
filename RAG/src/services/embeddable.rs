use std::sync::Arc;

use actix::{Addr, SyncArbiter};
use actix_web::{web::Payload, HttpResponse};
use mongodb::Collection;

use crate::{
    collection_values::media::Media, database::{mongodb::MongoClient, qdrant::MQdrantClient}, model::bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel}, EmbeddingActor, ExtractionActor
        }
};

pub struct EmbeddableService {
    qdrant: Arc<MQdrantClient>,
    vector_embedding_actor: Addr<EmbeddingActor>,
    media_collection: Collection<Media>,
    keyword_actor: Addr<ExtractionActor>,
}

impl EmbeddableService {
    pub fn new(
        qdrant: Arc<MQdrantClient>,
        mongo_client: Arc<MongoClient>
    ) -> Self {
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

    pub async fn upload(&self, payload: Payload) -> HttpResponse {
        //TODO
        todo!()        
    }
        

    fn calculate_embeddings_from_text(&self) {}
}
