use std::sync::Arc;

use actix::Message;
use actix_web::{HttpResponse, body::MessageBody, web::Json};
use anyhow::Result;
use bson::Document;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{collection_values::AsDocument, database::mongodb::MongoClient};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Position(i32, i32);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DraggableElement {
    id: Option<String>,
    position: Position,
}
impl AsDocument for DraggableElement {}
pub struct ElementService {
    collection: Collection<DraggableElement>,
}

impl ElementService {
    pub fn new(mongo_client: Arc<MongoClient>) -> Self {
        let mongo_connection = mongo_client.database("RAG").collection("Element");
        Self {
            collection: mongo_connection,
        }
    }

    pub async fn save_element(&self, draggable: DraggableElement) -> HttpResponse {
        return match self.collection.insert_one(&draggable).await {
            Ok(element) => HttpResponse::Ok().json(element.inserted_id),
            Err(_) => HttpResponse::InternalServerError().body("Failed to save element"),
        };
    }

    pub async fn get_element(&self, id: String) -> HttpResponse {
        let mut document = Document::new();
        document.insert("id", id.clone());
        return match self.collection.find_one(document).await {
            Ok(draggable) => HttpResponse::Ok().json(draggable.unwrap()),
            Err(_) => HttpResponse::NotFound().body(format!("Element with '{}' not found", id)),
        };
    }
}
