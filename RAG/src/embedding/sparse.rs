use embed_anything::embeddings::{embed::{Embedder, EmbedderBuilder}, local::text_embedding::ONNXModel};
use log::debug;

use crate::embedding::EmbeddingMessagesRequest;

pub struct SparseTextEmbeddingModel {
    model: Embedder,
}

impl SparseTextEmbeddingModel {
    pub fn new(identifier: Option<&str>) -> Self {
        Self {
            model: Embedder::from_pretrained_onnx(
                "sparse-bert",
                Some(ONNXModel::SPLADEPPENV1),
                None,
                None,
                None,
                None,
            )
            .unwrap(),
        } 
    }
}

impl SparseTextEmbeddingModel {
    pub async fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        let formatted_texts = msg
            .text
            .as_ref()
            .iter()
            .map(|str| str.as_str())
            .collect::<Vec<&str>>();
        let embeddings = match self.model.embed(&formatted_texts, None, None).await {
            Ok(embedding_result) => embedding_result
                .iter()
                .map(|embedding| embedding.to_dense().unwrap())
                .collect::<Vec<_>>(),
            Err(err) => {
                debug!("{:?}", err);
                vec![]
            }
        };
        embeddings
    }
}
