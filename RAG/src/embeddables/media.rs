use std::path::PathBuf;

use bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    timestamp: u64,
    len: u32,
}
