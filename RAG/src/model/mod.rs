use bson::Document;
use serde::Serialize;

pub mod pdf;
pub mod values;
pub mod bert_actors;

pub trait IntoDocument {
    fn into_document(self) -> Option<Document>
    where
        Self: std::marker::Sized + Serialize,
    {
        bson::to_bson(&self).unwrap().as_document().cloned()
    }


}
