
use std::fmt::Debug;

use actix_web::{web::Json, HttpResponse};
use mongodb::Collection;

use crate::embeddables::Embeddable;


pub trait EmbeddableMarker: Embeddable + Debug {}

#[derive(Debug)]
pub struct EmbeddableService {
    collection: Collection<Box<dyn EmbeddableMarker>>,
}

impl EmbeddableService {
   pub fn new(collection: Collection<Box<dyn EmbeddableMarker>>) -> Self {
      Self {
          collection
      }
   } 

   pub fn upload(&self, embeddable: Box<dyn Embeddable>) -> HttpResponse {
        HttpResponse::Ok().finish()
   }
}
