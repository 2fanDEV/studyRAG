use std::sync::Arc;

use actix::{Actor, SyncArbiter, SyncContext};
use actix_web::{
    post,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer,
};
use rust_bert::pipelines::keywords_extraction::{KeywordExtractionConfig, KeywordExtractionModel};
use RAG::{
    database::mongodb::MongoClient,
    model::pdf::Pdf,
    services::pdf_upload::{ActorTest, PdfService},
};

#[post("/pdf/upload")]
async fn pdf_upload(pdf: Json<Pdf>, pdf_service: Data<PdfService>) -> HttpResponse {
    pdf_service.store_pdf(pdf.into_inner()).await
}

pub struct AppState {
    pdf_service: PdfService,
}

pub fn create_pdf_service(mongo_client: Arc<MongoClient>) -> Data<PdfService> {
    let keywords_extract = SyncArbiter::start(1, || {
        let model = KeywordExtractionModel::new(KeywordExtractionConfig::default()).unwrap();
        ActorTest {
            keywords_extract: model,
        }
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
        .try_init()
        .unwrap();

    HttpServer::new(|| {
        let mongo_client = Arc::new(MongoClient::new().unwrap());
        let pdf_service = create_pdf_service(mongo_client.clone());
        App::new().app_data(pdf_service.clone()).service(pdf_upload)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
