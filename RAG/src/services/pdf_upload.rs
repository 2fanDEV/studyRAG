
use actix::{dev::MessageResponse, Actor, Addr, SyncContext};
use actix_web::{
    HttpResponse,
};
use log::debug;
use lopdf::Document;
use mongodb::{options::UpdateModifications, Collection};
use rust_bert::
    pipelines::keywords_extraction::KeywordExtractionModel
;

use crate::model::{bert_actors::{ActorTest, ExtractionText}, pdf::Pdf, values::Id, IntoDocument};


pub struct PdfService {
    pub collection: Collection<Pdf>,
    pub keywords_extractor: Addr<ActorTest>
}

impl PdfService {
    pub fn new(collection: Collection<Pdf>, keywords_extractor: Addr<ActorTest>) -> PdfService {
        PdfService {
            collection,
            keywords_extractor
        }
    }

    pub async fn store_pdf(&self, pdf: Pdf) -> HttpResponse {
        let document = match pdf.clone().into_document() {
            Some(doc) => doc,
            None => {
                return HttpResponse::InternalServerError().finish();
            }
        };
        let pdf_doc= Document::load_mem(&pdf.blob).unwrap();
        let collect =pdf_doc.get_pages().into_iter().map(|entry| entry.0).collect::<Vec<u32>>();
        println!("collect");
        let send = self.keywords_extractor.send(ExtractionText(pdf_doc.extract_text(&[1]).unwrap())).await.unwrap();
        debug!("{:?}", send);
        //self.collection.insert_one(pdf).await.unwrap();
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
