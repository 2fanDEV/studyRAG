use anyhow::{anyhow, Result};
use embed_anything::embeddings::{
    embed::{Embedder, EmbedderBuilder},
    local::text_embedding::ONNXModel,
};
use log::debug;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

use crate::embedding::{EmbeddingMessagesRequest, EmbeddingModel, ExtractionModel};

pub struct DenseEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl DenseEmbeddingModel {
    pub fn new(model_type: SentenceEmbeddingsModelType) -> Result<Self> {
        // change to match when you want to make it configurable from frontend
        let model = SentenceEmbeddingsBuilder::remote(model_type)
            .create_model()
            .unwrap();
        Ok(Self { model })
    }
}

impl EmbeddingModel for DenseEmbeddingModel {
    fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        let encode = self.model.encode(msg.text.as_ref()).unwrap();
        encode
    }
}


