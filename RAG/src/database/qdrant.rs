use std::io::Result;

use qdrant_client::{
    qdrant::{CreateCollectionBuilder, VectorParamsBuilder, VectorsConfig},
    Qdrant,
};

pub struct MQdrantClient {
    qdrant: Qdrant,
}

impl MQdrantClient {
    pub fn new() -> Result<MQdrantClient> {
        let endpoint = dotenv::var("QDRANT_ENDPOINT").unwrap();
        let qdrant = Qdrant::from_url(&endpoint).build().unwrap();
        Ok(Self { qdrant })
    }

    pub async fn create_collection(&mut self) {

    }
}
