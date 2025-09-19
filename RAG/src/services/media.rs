use std::sync::Arc;

use actix_multipart::{form::MultipartForm, Multipart};
use actix_web::HttpResponse;
use bson::doc;
use mongodb::Collection;

use crate::{
    collection_values::{AsDocument, media::MediaInformation},
    database::mongodb::MongoClient,
};

pub struct MultipartUploadInformation {
    pub id: String,
    pub bytes: Vec<u8>,
    pub received_chunk_indexes: i32,
    pub total_chunk_indexes: i32
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    
}

pub struct MediaService {
    pub collection: Collection<Box<MediaInformation>>,
    pub ongoing_multi_part_uploads: Vec<MultipartUploadInformation>
}

impl MediaService {
    pub fn new(mongo_client: Arc<MongoClient>) -> Self {
        Self {
            collection: mongo_client.database("RAG").collection("MediaInformation"),
            ongoing_multi_part_uploads: vec![]
        }
    }

    pub async fn upload(&self, media_information: MediaInformation) -> HttpResponse {
        let query_by_id = doc! { "id": &media_information.id()};
        match self.collection.find_one(query_by_id.clone()).await {
            Ok(found_media_information) => {
                return match self
                    .collection
                    .update_one(
                        query_by_id,
                        doc! { "$set":  &media_information.as_doc().unwrap()},
                    )
                    .await
                {
                    Ok(update_result) => HttpResponse::Ok().finish(),
                    Err(error) => HttpResponse::InternalServerError().finish(),
                };
            }
            Err(err) => {
                return match self
                    .collection
                    .insert_one(Box::new(media_information))
                    .await
                {
                    Ok(res) => HttpResponse::Ok().finish(),
                    Err(err) => HttpResponse::InternalServerError().finish(),
                };
            }
        }
    }

    pub fn upload_multi_part_file(&mut self, payload: Multipart) -> HttpResponse {


        HttpResponse::Ok().finish()
    }
}
