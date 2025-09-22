use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse,
};
use serde::Deserialize;
use serde_qs::actix::QsQuery;
use crate::{collection_values::media::MediaInformation, services::media::MediaService};

#[derive(Debug, MultipartForm)]
pub struct ChunkUploadForm {
    id: Text<String>,
    chunk_data: Bytes,
    chunk_index: Text<String>,
    total_chunks: Text<String>,
}

#[derive(Debug, Deserialize)]
pub struct IdQueryParam {
    pub ids: Vec<String>
}

#[get("file_information/get_by_ids")]
async fn get_file_information_by_ids(
    ids: QsQuery<IdQueryParam>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    let ids = ids.into_inner().ids;
    media_service.get_by_ids(&ids).await
}

#[post("file_information/save")]
async fn save_media_information(
    media_information: Json<MediaInformation>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    media_service.save(media_information.into_inner()).await
}

#[post("file_information/upload")]
async fn upload_media(
    MultipartForm(form): MultipartForm<ChunkUploadForm>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    let id = form.id.into_inner();
    let chunk_index = form.chunk_index.into_inner().parse::<u32>().unwrap();
    let chunk_data = &form.chunk_data.data;
    let total_chunks = form.total_chunks.into_inner().parse::<u32>().unwrap();
    media_service
        .upload_multi_part_file(id, chunk_data, chunk_index, total_chunks)
        .await
}
