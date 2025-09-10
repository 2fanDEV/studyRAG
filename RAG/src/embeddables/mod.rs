use bson::{DateTime, Uuid};
use rust_bert::pipelines::keywords_extraction::Keyword;

use crate::embeddables::{document::DocumentType, media::MediaType};

pub mod document;
pub mod media;

pub enum EmbeddableType {
    MediaType(MediaType),
    DocumentType(DocumentType)
}

#[typetag::serde]
pub trait Embeddable: Send + Sync {
    fn name(&self) -> &str;
    fn id(&self) -> Uuid;
    fn ty(&self) -> EmbeddableType;
    fn path(&mut self) -> &str;
    fn len(&self) -> u32;
    fn timestamp(&self) -> DateTime;
    fn tags(&self) -> Vec<Vec<Keyword>>;
}



