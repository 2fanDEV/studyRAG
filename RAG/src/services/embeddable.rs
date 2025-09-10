
use std::{fmt::Debug, sync::Arc};

use actix_web::HttpResponse;
use mongodb::Collection;

use crate::{database::qdrant::MQdrantClient, embeddables::Embeddable};


pub trait EmbeddableMarker: Embeddable + Debug {}

pub struct EmbeddableService {
    qdrant: Arc<MQdrantClient>,
    collection: Collection<Box<dyn EmbeddableMarker>>,
}

impl EmbeddableService {
   pub fn new(collection: Collection<Box<dyn EmbeddableMarker>>, qdrant: Arc<MQdrantClient>) -> Self {
      Self {
          collection,
          qdrant
      }
   }

   pub fn upload(&self, embeddable: Box<dyn Embeddable>) -> HttpResponse {
        HttpResponse::Ok().finish()
   }
}
