use std::marker::PhantomData;

use actix::Message;
use rust_bert::pipelines::keywords_extraction::Keyword;


pub mod bert_actors;
pub mod processer;

pub trait ResultMarker {}

pub struct BertRequest<ResType>
where
    ResType: ResultMarker,
{
    pub text: Vec<BertMessage>,
    pub _data: PhantomData<ResType>,
}

impl ResultMarker for Vec<Vec<f32>> {}
impl Message for BertRequest<Vec<Vec<f32>>> {
    type Result = Vec<Vec<f32>>;
}

impl ResultMarker for Vec<Vec<Keyword>> {}
impl Message for BertRequest<Vec<Vec<Keyword>>> {
    type Result = Vec<Vec<Keyword>>;
}

pub type EmbeddingMessagesRequest = BertRequest<Vec<Vec<f32>>>;
pub type ExtractionMessageRequest = BertRequest<Vec<Vec<Keyword>>>;

#[derive(Debug)]
pub struct BertMessage {
    pub text: String,
}

impl AsRef<str> for BertMessage {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

pub trait EmbeddingModel {
    fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>>;
}

pub trait ExtractionModel {
    fn process(&self, msg: ExtractionMessageRequest) -> Vec<Vec<Keyword>>;
}
