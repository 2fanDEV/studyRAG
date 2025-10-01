use std::{cell::RefCell, fmt::Pointer, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    dev::Service, http, middleware::Logger, web::{Data, PayloadConfig}, App, HttpServer
};
use log::debug;
use serde_qs::{actix::QsQueryConfig, Config};
use RAG::{
    database::{mongodb::MongoClient, qdrant::MQdrantClient},
    endpoints::{
        auth::exchange_token, element::{get_all_draggable, get_all_draggable_count, save_draggable}, embeddings::process_embeddings, media_information::{get_file_information_by_ids, save_media_information, upload_media}, query::send_query
    },
    services::{element::ElementService, embeddable::EmbeddableService, media::MediaService},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("lopdf", log::LevelFilter::Error)
        .try_init()
        .unwrap();

    HttpServer::new(|| {
        let qdrant = RefCell::new(MQdrantClient::new().unwrap());
        let mongo_client = Arc::new(MongoClient::new().unwrap());
        let serde_config = QsQueryConfig::default().qs_config(Config::new(5, false));

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .app_data(serde_config)
            .app_data(PayloadConfig::new(1024 * 1024))
            .app_data(Data::new(MediaService::new(mongo_client.clone())))
            .app_data(Data::new(ElementService::new(mongo_client.clone())))
            .app_data(Data::new(EmbeddableService::new(qdrant, mongo_client)))
            .service(save_media_information)
            .service(upload_media)
            .service(save_draggable)
            .service(get_all_draggable)
            .service(get_all_draggable_count)
            .service(get_file_information_by_ids)
            .service(process_embeddings)
            .service(send_query)
            .service(exchange_token)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
