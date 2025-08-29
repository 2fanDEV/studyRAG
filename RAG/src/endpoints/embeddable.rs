use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::{embeddables::Embeddable, services::embeddable::EmbeddableService};


#[post("embeddable/upload")]
pub fn upload<T>(embeddable: Json<T>, embeddable_service: Data<EmbeddableService>) -> HttpResponse {
    embeddable_service.upload(embeddable);

    HttpResponse::Ok().finish()
}
