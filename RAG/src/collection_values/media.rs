use std::path::PathBuf;

use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

use crate::collection_values::AsDocument;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LocationPath {
    Link(String),
    File(PathBuf),
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum MediaType {
    PDF,
    TXT,
    WORD,
    MP4,
    MP3,
    RAW,
    JPG,
    PNG,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaInformation {
    id: String,
    name: String,
    ty: MediaType,
    path: LocationPath,
    #[serde(rename = "len")]
    length: u32,
    #[serde(default)]
    timestamp: i64,
    #[serde(default)]
    tags: Vec<Vec<Keyword>>,
}

impl AsDocument for MediaInformation {}
impl MediaInformation {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn ty(&self) -> MediaType {
         self.ty
    }

    pub fn path(&self) -> LocationPath {
        self.path.clone()
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn timestamp(&self) -> bson::DateTime {
        bson::DateTime::from_millis(self.timestamp)
    }

    pub fn tags(&self) -> Vec<Vec<Keyword>> {
        self.tags.clone()
    }

}

