use actix_multipart::form::MultipartForm;
use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};
use log::debug;

use crate::{collection_values::media::MediaInformation, services::media::MediaService};

#[derive(Debug, MultipartForm)]
pub struct ChunkUploadForm {
    id: actix_multipart::form::json::Json<String>,
    chunk_data: actix_multipart::form::json::Json<Vec<u8>>,
    chunk_index: actix_multipart::form::json::Json<String>,
    total_chunks:  actix_multipart::form::json::Json<String>,
    file_name:  actix_multipart::form::json::Json<String>,
}

#[post("file_information/save")]
async fn save_media_information(
    media_information: Json<MediaInformation>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    media_service.upload(media_information.into_inner()).await
}

#[post("file_information/upload")]
async fn save_media(
    MultipartForm(form): MultipartForm<ChunkUploadForm>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    debug!("{:?}", form);
    let id = form.id.into_inner();
    let chunk_index = form.chunk_index.into_inner().parse::<u32>().unwrap();
    let total_chunks=form.total_chunks.into_inner().parse::<u32>().unwrap();
    let file_name= form.file_name.into_inner();
    media_service.upload_multi_part_file(id, file_name, &[], chunk_index, total_chunks).await
}
