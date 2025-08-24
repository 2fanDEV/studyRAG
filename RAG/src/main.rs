use std::{ops::Deref, sync::Arc};

use actix_web::{
    main, post,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer,
};
use serde::Serialize;
use RAG::{database::mongodb::MongoClient, model::pdf::Pdf, services::pdf_upload::PdfService};

#[post("/pdf/upload")]
async fn pdf_upload(pdf: Json<Pdf>, pdf_service: Data<PdfService>) -> HttpResponse {
    pdf_service.store_pdf(pdf.into_inner()).await
}

pub struct AppState {
    pdf_service: PdfService,
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mongo_client = Arc::new(MongoClient::new().unwrap());
        App::new()
            .app_data(PdfService::new(
                mongo_client.database("pdfService").collection("pdf"),
            ))
            .service(pdf_upload)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
