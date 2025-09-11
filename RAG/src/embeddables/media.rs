use std::path::PathBuf;

use bson::{DateTime, Uuid};
use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

use crate::embeddables::{Embeddable, EmbeddableType};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaType {
    MP4,
    MP3,
    PNG,
    JPEG,
    RAW
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    identifier: Uuid,
    name: String,
    ty: MediaType,
    path: PathBuf,
    timestamp: i64,
    length: u32,
    tags: Vec<Vec<Keyword>>
}

impl Embeddable for Media {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> Uuid {
        self.identifier
    }

    fn ty(&self) -> EmbeddableType {
        EmbeddableType::MediaType(self.ty)
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn len(&self) -> u32 {
        self.length
    }

    fn timestamp(&self) -> DateTime {
        bson::DateTime::from_millis(self.timestamp)
    }

    fn tags(&self) -> Vec<Vec<Keyword> >  {
        self.tags.clone()
    }

    #[doc(hidden)]
    fn typetag_name(&self) ->  &'static str {
        "Document"
    }

    #[doc(hidden)]
    fn typetag_deserialize(&self) {
        "Document";
    }
}
