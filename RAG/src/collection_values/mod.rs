use anyhow::Result;
use bson::Document;
use serde::Serialize;

pub mod media;

pub trait AsDocument: Serialize {
    fn as_doc(&self) -> Result<Document> {
        Ok(bson::to_document(self)?)
    }
}
