use std::future::{self, ready, Ready};

use actix::{Actor, Handler, SyncContext};
use rust_bert::pipelines::{
    keywords_extraction::Keyword, sentence_embeddings::SentenceEmbeddingsModelType,
};

use crate::embedding::{
    embedding_actors::embedding_models::{DenseEmbeddingModel},
    EmbeddingMessagesRequest, EmbeddingModel, ExtractionMessageRequest, ExtractionModel,
};

pub mod embedding_models;

pub enum SpecifiedModelType {
    DENSE(SentenceEmbeddingsModelType),
}

pub enum SpecifiedEmbeddingModel {
    Dense(DenseEmbeddingModel),
}

pub struct EmbeddingActor {
    model: SpecifiedEmbeddingModel,
}

impl Handler<EmbeddingMessagesRequest> for EmbeddingActor {
    type Result = Vec<Vec<f32>>;
    fn handle(&mut self, msg: EmbeddingMessagesRequest, ctx: &mut Self::Context) -> Self::Result {
        self.process(msg)
    }
}

impl EmbeddingActor {
    pub fn new(identifier: SpecifiedModelType) -> Self {
        let model = match identifier {
            SpecifiedModelType::DENSE(model_type) => {
                SpecifiedEmbeddingModel::Dense(DenseEmbeddingModel::new(model_type).unwrap())
            }
        };
        Self { model }
    }

    pub fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        match &self.model {
            SpecifiedEmbeddingModel::Dense(dense_embedding_model) => {
                dense_embedding_model.process(msg)
            }
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
