use std::{fmt::Debug, fs, sync::Arc};

use actix::{Actor, Addr, SyncArbiter};
use actix_web::HttpResponse;
use mongodb::Collection;

use crate::{
    collection_values::embeddables::{document::DocumentType, media::MediaType, Embeddable, EmbeddableType, LocationPath}, database::qdrant::MQdrantClient, model::{
        bert_actors::{
            bert_models::{KeywordExtractionModel, VectorEmbeddingModel},
            EmbeddingActor, ExtractionActor,
        },
        split_by_context_size,
    }
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

    pub async fn upload(&self, embeddable: Box<dyn Embeddable>) -> HttpResponse {
        match embeddable.path() {
            LocationPath::Link(url) => {
                let url = url;
            }
            LocationPath::File(path_buf) => {
                let path = path_buf;
                match embeddable.ty() {
                    EmbeddableType::MediaType(media_type) => match media_type {
                        MediaType::MP4 => todo!(),
                        MediaType::MP3 => todo!(),
                        MediaType::PNG => todo!(),
                        MediaType::JPEG => todo!(),
                        MediaType::RAW => todo!(),
                    },
                    EmbeddableType::DocumentType(document_type) => {
                        match document_type {
                            DocumentType::PDF => {
                                let pdf = lopdf::Document::load(path).unwrap();
                                let pages = pdf
                                    .get_pages()
                                    .into_iter()
                                    .map(|(page_num, page_id)| page_num)
                                    .collect::<Vec<_>>();
                                let extract_text = pdf.extract_text(&pages).unwrap();
                                let send = self
                                    .keyword_actor
                                    .send(split_by_context_size(extract_text, 0, 20))
                                    .await
                                    .unwrap();
                            }
                            DocumentType::TXT => todo!(),
                        }
                    }
                }
            }
        }
        HttpResponse::Ok().finish()
    }

    fn calculate_embeddings_from_text(&self) {}
}
