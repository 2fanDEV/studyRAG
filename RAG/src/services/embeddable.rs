use std::{fmt::Debug, sync::Arc};

use actix::{Actor, Addr, SyncArbiter};
use actix_web::HttpResponse;
use mongodb::Collection;

use crate::{
    database::qdrant::MQdrantClient,
    embeddables::{media::MediaType, Embeddable},
    model::bert_actors::{
        bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
        EmbeddingActor, ExtractionActor,
    },
};

pub trait EmbeddableMarker: Embeddable + Debug {}

pub struct EmbeddableService {
    qdrant: Arc<MQdrantClient>,
    collection: Collection<Box<dyn EmbeddableMarker>>,
    vector_embedding_actor: Addr<EmbeddingActor>,
    keyword_actor: Addr<ExtractionActor>,
}

impl EmbeddableService {
    pub fn new(
        collection: Collection<Box<dyn EmbeddableMarker>>,
        qdrant: Arc<MQdrantClient>,
    ) -> Self {
        let embedding_actor = SyncArbiter::start(1, || {
            EmbeddingActor::new(Box::new(VectorEmbeddingModel::new().unwrap()))
        });
        let extracting_actor = SyncArbiter::start(1, || {
            ExtractionActor::new(Box::new(KeywordExtractionModel::new().unwrap()))
        });
        Self {
            collection,
            qdrant,
            vector_embedding_actor: embedding_actor,
            keyword_actor: extracting_actor,
        }
    }

    pub fn upload(&self, embeddable: Box<dyn Embeddable>) -> HttpResponse {
        let ty = embeddable.ty();
        let path = embeddable.path();

        match path {
            crate::embeddables::LocationPath::Link(url) => todo!(),
            crate::embeddables::LocationPath::File(path_buf) => todo!(),
        }



        HttpResponse::Ok().finish()
    }
}
