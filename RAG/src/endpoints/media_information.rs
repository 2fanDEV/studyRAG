use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{collection_values::media::{self, MediaInformation}, services::media::MediaService};

#[post("file_information/save")]
async fn save_media_information(
    media_information: Json<MediaInformation>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    media_service.upload(media_information.into_inner()).await
}


async fn save_media(mut payload: Multipart, media_service: Data<MediaService>) -> HttpResponse {
    media_service.upload_multi_part_file(payload)
}
