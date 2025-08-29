use std::path::PathBuf;

use bson::Uuid;
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DocumentType {
    PDF,
    TXT,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Document {
    identifier: Uuid,
    name: String,
    ty: DocumentType,
    path: PathBuf,
    length: u32,
    timestamp: u64,
    tags: Vec<Vec<Keyword>>,
}
