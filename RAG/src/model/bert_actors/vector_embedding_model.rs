use actix::{Actor, SyncArbiter};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

use crate::model::bert_actors::{EmbeddingMessages, EmbeddingModel};

pub struct VectorEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl EmbeddingModel for VectorEmbeddingModel {
    fn process(&self, msg: EmbeddingMessages) -> Vec<Vec<f32>> {
        let encode = self.model.encode(&msg.full_text).unwrap();
        encode
    }
}
