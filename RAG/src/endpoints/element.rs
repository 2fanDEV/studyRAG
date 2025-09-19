use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse};
use log::debug;

use crate::services::element::{DraggableElement, ElementService};

#[post("draggable/save")]
pub async fn save_draggable(draggable: Json<DraggableElement>, element_service: Data<ElementService> ) -> HttpResponse {
    return element_service.save_element(draggable.into_inner()).await
}

#[get("draggable/count")]
pub async fn get_all_draggable_count(element_service: Data<ElementService>) -> HttpResponse {
    element_service.get_count().await
}

#[get("draggable/all/{page}")]
pub async fn get_all_draggable(page: Path<u64>, element_service: Data<ElementService>) -> HttpResponse {
    debug!("GetAll");
    element_service.get_all(page.to_owned()).await
}