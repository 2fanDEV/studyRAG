use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    http,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use RAG::{
    database::{mongodb::MongoClient, qdrant::MQdrantClient}, endpoints::{element::save_draggable, embeddable::embeddable_upload}, services::{element::ElementService, embeddable::EmbeddableService}
};

pub struct AppState {
    embeddable_service: EmbeddableService,
}

pub fn create_embeddable_service(
    mongo_client: Arc<MongoClient>,
    qdrant: Arc<MQdrantClient>,
) -> Data<EmbeddableService> {
    Data::new(EmbeddableService::new(
        mongo_client.database("RAG").collection("embeddable"),
        qdrant
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("lopdf", log::LevelFilter::Error)
        .try_init()
        .unwrap();

    HttpServer::new(|| {
        let qdrant = Arc::new(MQdrantClient::new().unwrap());
        let mongo_client = Arc::new(MongoClient::new().unwrap());
        let embeddable_service = create_embeddable_service(mongo_client.clone(), qdrant);
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .app_data(embeddable_service.clone())
            .app_data(ElementService::new(mongo_client.clone()))
            .service(embeddable_upload)
            .service(save_draggable)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
