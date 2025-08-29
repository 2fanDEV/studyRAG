use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

pub struct VectorEmbeddingModel {
    model: SentenceEmbeddingsModel
}
