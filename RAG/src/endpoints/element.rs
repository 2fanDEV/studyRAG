use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::services::element::{DraggableElement, ElementService};

#[post("draggable/save")]
pub async fn save_draggable(draggable: Json<DraggableElement>, element_service: Data<ElementService> ) -> HttpResponse {
    println!("HELLLOOOO");
    element_service.save_element(draggable.into_inner()).await
}