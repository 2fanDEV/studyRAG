use std::sync::Arc;

use actix_web::HttpResponse;
use bson::{Document, Uuid, doc};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{boxed_values::Count, collection_values::AsDocument, database::mongodb::MongoClient};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

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

    pub async fn get_count(&self) -> HttpResponse {
        let count = self.collection.count_documents(doc! {}).await.unwrap();
        HttpResponse::Ok().json(Count { count })
    }

    pub async fn get_all(&self, page: u64) -> HttpResponse {
        let mut draggables = vec![];
        let page_size = 50;
        let mut cursor = self
            .collection
            .find(doc! {})
            .batch_size(50)
            .skip(page * page_size)
            .await
            .unwrap();
        while cursor.advance().await.unwrap() {
            let current = cursor.deserialize_current().unwrap();
            draggables.push(current);
        }

        HttpResponse::Ok().json(draggables)
    }

    pub async fn save_element(&self, mut draggable: DraggableElement) -> HttpResponse {
        let by_id = doc! { "id": &draggable.id};
        return match self.collection.find_one(doc! { "id": &draggable.id}).await {
            Ok(result) => match result {
                Some(_draggable_element) => {
                    match self
                        .collection
                        .update_one(by_id, doc! { "$set": draggable.as_doc().unwrap() })
                        .await
                    {
                        Ok(res) => HttpResponse::Ok().json(res),
                        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                    }
                }
                None => {
                    draggable.id = Some(Uuid::new().to_string());
                    return match self.collection.insert_one(&draggable).await {
                        Ok(element) => HttpResponse::Ok().json(draggable.id),
                        Err(_) => {
                            HttpResponse::InternalServerError().body("Failed to save element!")
                        }
                    };
                }
            },
            Err(_) => HttpResponse::InternalServerError().finish(),
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
