use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use serde::Deserialize;

use crate::services::embeddable::EmbeddableService;

#[derive(Debug, Clone, Deserialize)]
pub struct QueryRequest {
    pub text: String,
}

#[post("send_query")]
pub async fn send_query(
    query: Json<QueryRequest>,
    embeddable_service: Data<EmbeddableService>,
) -> HttpResponse {
    embeddable_service.execute_query(query.into_inner()).await
}
