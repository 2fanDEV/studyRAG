use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    embeddables::Embeddable,
    services::embeddable::EmbeddableService,
};

#[post("embeddable/upload")]
async fn embeddable_upload(
    embeddable: Json<Box<dyn Embeddable>>,
    embeddable_service: Data<EmbeddableService>,
) -> HttpResponse {
    embeddable_service.upload(embeddable.into_inner())
}
