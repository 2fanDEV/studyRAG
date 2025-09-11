use actix::{Actor, Context, Handler, Message, SyncContext};
use rust_bert::pipelines::keywords_extraction::Keyword;

use crate::model::{
    EmbeddingMessagesRequest, EmbeddingModel, ExtractionMessageRequest, ExtractionModel,
};

pub mod bert_models;

pub struct EmbeddingActor {
    model: Box<dyn EmbeddingModel>,
}

impl Handler<EmbeddingMessagesRequest> for EmbeddingActor {
    type Result = Result<Vec<Vec<f32>>, std::io::Error>;
    fn handle(&mut self, msg: EmbeddingMessagesRequest, ctx: &mut Self::Context) -> Self::Result {
        Ok(self.model.process(msg))
    }
}


impl EmbeddingActor {
    pub fn new(model: Box<dyn EmbeddingModel>) -> Self {
        Self {
            model
        } 
    }
}

impl Actor for EmbeddingActor {
    type Context = SyncContext<Self>;
}

pub struct ExtractionActor {
    model: Box<dyn ExtractionModel>,
}

impl ExtractionActor {
    pub fn new(model: Box<dyn ExtractionModel>) -> Self {
        Self {
            model
        } 
    }
}

impl Actor for ExtractionActor {
    type Context = SyncContext<Self>;
}

impl Handler<ExtractionMessageRequest> for ExtractionActor {
    type Result = Result<Vec<Vec<Keyword>>, std::io::Error>;

    fn handle(&mut self, msg: ExtractionMessageRequest, ctx: &mut Self::Context) -> Self::Result {
        Ok(self.model.process(msg))
    }
}
