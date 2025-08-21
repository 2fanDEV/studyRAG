use bson::Document;
use serde::{Deserialize, Serialize};

use crate::model::{values::Id, IntoDocument};

#[derive(Serialize, Deserialize, Clone)]
pub struct PDFType {
    ty: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pdf {
    pub id: Id,
    file_name: String,
    page_length: u32,
    length: u32,
    blob: Vec<u8>,
    ty: PDFType,
}

impl IntoDocument for Pdf {}
