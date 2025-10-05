use actix::{Actor, Handler, SyncContext};
use log::debug;
use rust_bert::pipelines::keywords_extraction::Keyword;

use crate::embedding::{
    EmbeddingMessagesRequest, EmbeddingModel, ExtractionMessageRequest, ExtractionModel,
};

pub mod embedding_models;

pub struct EmbeddingActor {
    model: Box<dyn EmbeddingModel>,
}

impl Handler<EmbeddingMessagesRequest> for EmbeddingActor {
    type Result = Vec<Vec<f32>>;
    fn handle(&mut self, msg: EmbeddingMessagesRequest, ctx: &mut Self::Context) -> Self::Result {
        let res = self.model.process(msg);
        debug!("res_len={:?}", res.len());
        res
    }
}

impl EmbeddingActor {
    pub fn new(model: Box<dyn EmbeddingModel>) -> Self {
        Self { model }
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
        Self { model }
    }
}

impl Actor for ExtractionActor {
    type Context = SyncContext<Self>;
}

impl Handler<ExtractionMessageRequest> for ExtractionActor {
    type Result = Vec<Vec<Keyword>>;

    fn handle(&mut self, msg: ExtractionMessageRequest, ctx: &mut Self::Context) -> Self::Result {
        self.model.process(msg)
    }
}
