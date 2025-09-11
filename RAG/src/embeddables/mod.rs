use std::path::PathBuf;

use bson::{DateTime, Uuid};
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

use crate::embeddables::{document::DocumentType, media::MediaType};

pub mod document;
pub mod media;

pub enum EmbeddableType {
    MediaType(MediaType),
    DocumentType(DocumentType)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LocationPath {
    Link(String),
    File(PathBuf)
}

#[typetag::serde]
pub trait Embeddable: Send + Sync {
    fn name(&self) -> &str;
    fn id(&self) -> Uuid;
    fn ty(&self) -> EmbeddableType;
    fn path(&self) -> LocationPath;
    fn len(&self) -> u32;
    fn timestamp(&self) -> DateTime;
    fn tags(&self) -> Vec<Vec<Keyword>>;
}



