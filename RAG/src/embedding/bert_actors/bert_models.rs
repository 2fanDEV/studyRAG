use anyhow::Result;
use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionConfig,
    sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModel},
};

use crate::embedding::{bert_actors::EmbeddingModel, EmbeddingMessagesRequest, ExtractionModel};

pub struct VectorEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl EmbeddingModel for VectorEmbeddingModel {
    fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        let encode = self.model.encode(&msg.text).unwrap();
        encode
    }
}

impl VectorEmbeddingModel {
    pub fn new() -> Result<Self> {
        let model = SentenceEmbeddingsBuilder::remote(
            rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType::SentenceT5Base,
        )
        .create_model()?;
     Ok(Self { model })
    }
}

pub struct KeywordExtractionModel {
    model: rust_bert::pipelines::keywords_extraction::KeywordExtractionModel<'static>,
}

impl ExtractionModel for KeywordExtractionModel {
    fn process(
        &self,
        msg: crate::embedding::ExtractionMessageRequest,
    ) -> Vec<Vec<rust_bert::pipelines::keywords_extraction::Keyword>> {
        let keywords = self.model.predict(&msg.full_text).unwrap();
        keywords
    }
}

impl KeywordExtractionModel {
    pub fn new() -> Result<Self> {
        Ok(Self {
            model: rust_bert::pipelines::keywords_extraction::KeywordExtractionModel::new(
                KeywordExtractionConfig::default(),
            )?,
        })
    }
}
