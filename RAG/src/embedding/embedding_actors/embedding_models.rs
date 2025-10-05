use anyhow::Result;
use embed_anything::{embed_query, embeddings::embed::{Embedder, EmbedderBuilder}};
use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionConfig,
    sentence_embeddings::{
        SentenceEmbeddingsBuilder, SentenceEmbeddingsConfig, SentenceEmbeddingsModel,
    },
};

use crate::embedding::{EmbeddingMessagesRequest, EmbeddingModel, ExtractionModel};

pub struct VectorEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl EmbeddingModel for VectorEmbeddingModel {
    async fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        let encode = self.model.encode(msg.text.as_ref()).unwrap();
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
        let keywords = self.model.predict(msg.text.as_ref()).unwrap();
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

pub struct SparseTextEmbeddingModel {
    model: Embedder,
}

impl SparseTextEmbeddingModel {
    pub fn new() -> Self {
        Self {
            model: EmbedderBuilder::new()
                .model_architecture("sparse-bert")
                .model_id(Some("prithivida/Splade_PP_en_v2"))
                .revision(None)
                .from_pretrained_hf()
                .unwrap(),
        }
    }
}


impl EmbeddingModel for SparseTextEmbeddingModel {
    async fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>> {
        let formatted_texts = msg.text.as_ref().iter().map(|str| str.as_str()).collect::<Vec<&str>>();
        let embeddings = match self.model.embed(&formatted_texts, None , None).await {
            Ok(embedding_result) => { embedding_result[0].to_dense().unwrap(); vec![] },
            Err(err) => vec![],
        };
        embeddings
    }
}
