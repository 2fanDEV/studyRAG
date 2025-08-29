use bson::{DateTime, Uuid};
use rust_bert::pipelines::keywords_extraction::Keyword;

use crate::embeddables::document::DocumentType;


pub mod document;
pub mod media;

pub trait Embeddable: Send + Sync {
    fn name(&self) -> String;
    fn id(&self) -> Uuid;
    fn ty(&self) -> DocumentType;
    fn path(&self) -> String;
    fn len(&self) -> u32;
    fn timestamp(&self) -> DateTime;
    fn tags(&self) -> Vec<Vec<Keyword>>;
}



