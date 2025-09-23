use actix_web::{
    post,
    web::{Data, Json, Query},
    HttpResponse,
};
use futures_util::AsyncReadExt;
use serde::Deserialize;

use crate::services::{embeddable::EmbeddableService, media::MediaService};

#[derive(Clone, Deserialize)]
pub struct IdQuery {
    id: String
}

#[post("/create_embedding")]
pub async fn process_embeddings(
    id: Query<IdQuery>,
    embeddings_service: Data<EmbeddableService>,
    media_service: Data<MediaService>,
) -> HttpResponse {
    return match media_service.get_gridfs_file_by_id(id.clone().into_inner().id).await {
        Ok(stream) => {
            return match stream.await {
                Ok(mut gridfs_stream) => {
                    let mut buf = vec![];
                    gridfs_stream.read_to_end(&mut buf).await.unwrap();
                    match embeddings_service.upload(id.into_inner().id,buf).await {
                        Ok(res) => res,
                        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
                    }
                }
                Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
            };
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    };
}
