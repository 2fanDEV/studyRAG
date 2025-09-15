use anyhow::Result;
use bson::Document;
use serde::Serialize;

pub mod embeddables;

pub trait AsDocument : Serialize {
    fn as_doc(&self) -> Result<Document> {
        Ok(bson::to_document(self)?)
    }
}



