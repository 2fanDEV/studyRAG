use std::sync::Arc;

use RAG::{
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    endpoints::{
        element::{get_all_draggable, get_all_draggable_count, save_draggable},
        media_information::{save_media_information},
    },
    services::{element::ElementService, media::MediaService},
};
use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger, web::Data};

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
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .app_data(Data::new(MediaService::new(mongo_client.clone())))
            .app_data(Data::new(ElementService::new(mongo_client.clone())))
            .service(save_media_information)
            .service(save_draggable)
            .service(get_all_draggable)
            .service(get_all_draggable_count)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
