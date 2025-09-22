use std::{
    cell::RefCell,
    sync::Arc,
};

use actix_web::HttpResponse;
use bson::doc;
use log::debug;
use mongodb::Collection;

use crate::{
    collection_values::{
        media::{Media, MediaInformation}, AsDocument
    },
    database::mongodb::MongoClient,
};

pub struct MultipartUploadInformation {
    pub id: String,
    pub bytes: Vec<u8>,
    pub chunk_index: u32,
    pub received_chunk_indexes: u32,
    pub total_chunk_indexes: u32,
}

pub struct MediaService {
    pub media_information_collection: Collection<MediaInformation>,
    pub media_collection: Collection<Media>,
    pub ongoing_multi_part_uploads: RefCell<Vec<MultipartUploadInformation>>,
}

impl MediaService {
    pub fn new(mongo_client: Arc<MongoClient>) -> Self {
        Self {
            media_information_collection: mongo_client.database("RAG").collection("MediaInformation"),
            media_collection: mongo_client.database("RAG").collection("Media"),
            ongoing_multi_part_uploads: RefCell::new(vec![]),
        }
    }

    pub async fn save(&self, media_information: MediaInformation) -> HttpResponse {
        debug!("{:?}", media_information);
        let query_by_id = doc! { "id": &media_information.id()};
        return match self.media_information_collection.find_one(query_by_id.clone()).await {
            Ok(found_media_information) => match found_media_information {
                Some(_media_info) => {
                    match self
                        .media_information_collection
                        .update_one(
                            query_by_id,
                            doc! { "$set":  &media_information.as_doc().unwrap()},
                        )
                        .await
                    {
                        Ok(update_result) => HttpResponse::Ok().json(update_result),
                        Err(_error) => HttpResponse::InternalServerError().finish(),
                    }
                }
                None => {
                    match self
                        .media_information_collection
                        .insert_one(&media_information)
                        .await
                    {
                        Ok(res) => HttpResponse::Ok().json(res),
                        Err(_err) => HttpResponse::InternalServerError().finish(),
                    }
                }
            },
            Err(err) => HttpResponse::InternalServerError().finish(),
        };
    }

    pub async fn upload_multi_part_file(
        &self,
        id: String,
        chunk_data: &[u8],
        chunk_index: u32,
        total_chunks: u32,
    ) -> HttpResponse {
        let mut result = HttpResponse::Ok().finish();
        let count_uploads = (self
            .ongoing_multi_part_uploads
            .borrow()
            .iter()
            .filter(|item| item.id.eq(&id))
            .collect::<Vec<_>>()
            .len()
            + 1) as u32;

        self.ongoing_multi_part_uploads
            .borrow_mut()
            .push(MultipartUploadInformation {
                id: id.clone(),
                bytes: chunk_data.to_vec(),
                received_chunk_indexes: count_uploads,
                chunk_index: chunk_index,
                total_chunk_indexes: total_chunks,
            });

        if count_uploads.eq(&total_chunks) {
            let bytes = self.get_bytes_of_multipart(&id);
            let media = Media {
                id: id.clone(),
                bytes,
            };
            result = match self
                .media_collection
                .insert_one(&media)
                .await
            {
                Ok(elem) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().json("Yeah about that.."),
            };
            self.ongoing_multi_part_uploads
                .borrow_mut()
                .retain(|item| !item.id.eq(&id));
        }
        result
    }

    fn get_bytes_of_multipart(&self, id: &str) -> Vec<u8> {
        let mut uploads = self
            .ongoing_multi_part_uploads
            .borrow_mut()
            .iter()
            .filter(|item| item.id.eq(&id))
            .map(|item| (item.chunk_index, item.id.clone(), item.bytes.clone()))
            .collect::<Vec<_>>();
        uploads.sort_by(|item1, item2| item2.0.cmp(&item1.0));
        uploads
            .iter()
            .flat_map(|item| item.2.clone())
            .collect::<Vec<_>>()
    }
}
