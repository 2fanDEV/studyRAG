use bson::Uuid;
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

use crate::collection_values::embeddables::{Embeddable, EmbeddableType, LocationPath};


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
    path: LocationPath,
    length: u32,
    timestamp: i64,
    tags: Vec<Vec<Keyword>>,
}

impl Embeddable for Document {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> Uuid {
        self.identifier
    }

    fn ty(&self) -> EmbeddableType {
        EmbeddableType::DocumentType(self.ty)
    }

    fn path(&self) -> LocationPath {
        self.path.clone()
    }

    fn len(&self) -> u32 {
        self.length
    }

    fn timestamp(&self) -> bson::DateTime {
        bson::DateTime::from_millis(self.timestamp)
    }

    fn tags(&self) -> Vec<Vec<Keyword>> {
        self.tags.clone()
    }

    #[doc(hidden)]
    fn typetag_name(&self) -> &'static str {
        "Document"
    }

    #[doc(hidden)]
    fn typetag_deserialize(&self) {
        "Document";
    }
}
