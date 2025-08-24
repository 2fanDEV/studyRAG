use actix_web::{
    web::{Json, JsonBody},
    HttpResponse,
};
use bson::{doc, Document};
use mongodb::{options::UpdateModifications, Collection, Database};

use crate::model::{pdf::Pdf, values::Id, IntoDocument};

pub struct PdfService {
    pub collection: Collection<Pdf>,
}

impl PdfService {
    pub fn new(collection: Collection<Pdf>) -> PdfService {
        PdfService { collection }
    }

    pub async fn store_pdf(&self, pdf: Pdf) -> HttpResponse {
        let document = match pdf.clone().into_document() {
            Some(doc) => doc,
            None => {
                return HttpResponse::InternalServerError().finish();
            }
        };
        self.collection.insert_one(pdf).await.unwrap();
        HttpResponse::Ok().finish()
    }

    pub async fn update_pdf(&self, pdf: Pdf) -> HttpResponse {
        let document = match self
            .collection
            .find_one(pdf.id.clone().into_document().unwrap())
            .await
        {
            Ok(pdf) => match pdf {
                Some(pdf) => pdf.into_document().unwrap(),
                None => return HttpResponse::NotFound().finish(),
            },
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        let update_one = self
            .collection
            .update_one(
                document,
                UpdateModifications::Document(pdf.into_document().unwrap()),
            )
            .await
            .unwrap();

        HttpResponse::Ok().json(update_one)
    }

    pub fn retrieve_pdf(&self, id: Id) {}
}
