use std::path::PathBuf;

use rust_bert::pipelines::keywords_extraction::Keyword;
use serde::{Deserialize, Serialize};

use crate::collection_values::AsDocument;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LocationPath {
    Link(String),
    File(PathBuf),
}

impl Default for LocationPath {
    fn default() -> Self {
        LocationPath::File(PathBuf::from("lelele"))
    }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub id: String,
    pub bytes: Vec<u8>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaInformation {
    pub id: String,
    pub name: String,
    pub ty: MediaType,
    #[serde(skip)]
    pub path: LocationPath,
    #[serde(skip)]
    pub bytes: Vec<u8>,
    #[serde(rename = "len")]
    pub length: u32,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub tags: Vec<Vec<Keyword>>,
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

