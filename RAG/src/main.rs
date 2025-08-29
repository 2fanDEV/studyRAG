use std::sync::Arc;

use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::{
    http,
    middleware::Logger,
    post,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer,
};
use log::debug;
use rust_bert::pipelines::keywords_extraction::{KeywordExtractionConfig, KeywordExtractionModel};
use RAG::{
    database::mongodb::MongoClient,
    model::{bert_actors::extract_actors::ActorTest, pdf::PdfDocumentDto},
};

#[post("/pdf/upload")]
async fn pdf_upload(pdf: Json<PdfDocumentDto>, pdf_service: Data<>) -> HttpResponse {
    debug!("LOL");
    pdf_service.store_pdf(pdf.into_inner().into()).await
}

pub struct AppState {
    pdf_service: PdfService,
}

pub fn create_pdf_service(mongo_client: Arc<MongoClient>) -> Data<PdfService> {
    let keywords_extract = SyncArbiter::start(1, || {
        let model = KeywordExtractionModel::new(KeywordExtractionConfig::default()).unwrap();
        ActorTest { model }
    });

    Data::new(PdfService::new(
        mongo_client.database("pdfService").collection("pdf"),
        keywords_extract,
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
        let mongo_client = Arc::new(MongoClient::new().unwrap());
        let pdf_service = create_pdf_service(mongo_client.clone());
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .app_data(web::JsonConfig::default().limit(50 * 1024 * 1024))
            .app_data(pdf_service.clone())
            .service(pdf_upload)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
