use crate::{
    collection_values::{
        media::{Media, MediaInformation},
        AsDocument,
    },
    database::mongodb::MongoClient,
};
use actix_web::HttpResponse;
use anyhow::{anyhow, Result};
use bson::doc;
use futures_util::io::AsyncWriteExt;
use mongodb::{
    action::gridfs::OpenDownloadStream, gridfs::GridFsBucket, options::GridFsBucketOptions,
    Collection,
};
use std::{cell::RefCell, sync::Arc};

pub struct MultipartUploadInformation {
    pub id: String,
    pub bytes: Vec<u8>,
    pub chunk_index: u32,
    pub received_chunk_indexes: u32,
    pub total_chunk_indexes: u32,
}

pub struct MediaService {
    pub media_information_collection: Collection<MediaInformation>,
    pub media_bucket: GridFsBucket,
    pub ongoing_multi_part_uploads: RefCell<Vec<MultipartUploadInformation>>,
}

impl MediaService {
    pub fn new(mongo_client: Arc<MongoClient>) -> Self {
        Self {
            media_information_collection: mongo_client
                .database("RAG")
                .collection("MediaInformation"),
            media_bucket: mongo_client.database("RAG").gridfs_bucket(
                GridFsBucketOptions::builder()
                    .bucket_name(Some("Media".to_string()))
                    .build(),
            ),
            ongoing_multi_part_uploads: RefCell::new(vec![]),
        }
    }

    pub async fn save(&self, media_information: MediaInformation) -> HttpResponse {
        let query_by_id = doc! { "id": &media_information.id()};
        return match self
            .media_information_collection
            .find_one(query_by_id.clone())
            .await
        {
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

    pub async fn get_by_ids(&self, ids: &[String]) -> HttpResponse {
        return match self
            .media_information_collection
            .find(doc! { "id": { "$in": ids}})
            .await
        {
            Ok(mut cursor) => {
                let mut media_informations = vec![];
                while cursor.advance().await.unwrap() {
                    let current = cursor.deserialize_current().unwrap();
                    media_informations.push(current);
                }
                return HttpResponse::Ok().json(media_informations);
            }
            Err(_) => HttpResponse::Ok().finish(),
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

            result = match self.media_bucket.open_upload_stream(&id).await {
                Ok(mut stream) => {
                    stream.write_all(&bytes[..]).await.unwrap();
                    stream.close().await.unwrap();
                    HttpResponse::Ok().finish()
                }
                Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
            };

            self.ongoing_multi_part_uploads
                .borrow_mut()
                .retain(|item| !item.id.eq(&id));
        }
        result
    }

    pub async fn get_gridfs_file_by_id(&self, id: String) -> Result<OpenDownloadStream> {
        let object_id = match self.media_bucket.find_one(doc! { "filename": id }).await {
            Ok(file_collection) => match file_collection {
                Some(collection) => Some(collection.id),
                None => None,
            },
            Err(err) => None,
        };
        match object_id {
            Some(id) => Ok(self.media_bucket.open_download_stream(id)),
            None => Err(anyhow!("No object id found")),
        }
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
