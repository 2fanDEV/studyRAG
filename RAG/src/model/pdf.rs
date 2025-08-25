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
    pub file_name: String,
    pub page_length: u32,
    pub length: u32,
    pub blob: Vec<u8>,
    pub ty: PDFType,
}

impl IntoDocument for Pdf {}
