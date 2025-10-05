use std::marker::PhantomData;

use actix::Message;
use rust_bert::pipelines::keywords_extraction::Keyword;


pub mod processer;
pub mod sparse;
pub mod embedding_actors;

pub trait ResultMarker {}

#[derive(Debug)]
pub struct BertRequest<ResType>
where
    ResType: ResultMarker,
{
    pub text: EmbeddingMessage,
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
pub struct EmbeddingMessage {
    pub text: Vec<String>,
}

impl AsRef<Vec<String>> for EmbeddingMessage {
    fn as_ref(&self) -> &Vec<String> {
        &self.text
    }
}

pub trait EmbeddingModel {
    fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>>;
}

pub trait ExtractionModel {
    fn process(&self, msg: ExtractionMessageRequest) -> Vec<Vec<Keyword>>;
}
