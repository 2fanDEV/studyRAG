use std::ops::Deref;

use anyhow::Result;
use qdrant_client::{
    qdrant::{CreateCollectionBuilder, VectorsConfigBuilder},
    Qdrant,
};

pub struct MQdrantClient {
    qdrant: Qdrant,
    all_collections: Vec<String>,
}

impl Deref for MQdrantClient {
    type Target = Qdrant;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl MQdrantClient {
    pub fn new() -> Result<MQdrantClient> {
        let endpoint = dotenv::var("QDRANT_ENDPOINT").unwrap();
        let qdrant = Qdrant::from_url(&endpoint).build().unwrap();
        Ok(Self {
            qdrant,
            all_collections: vec![],
        })
    }

    pub async fn create_default_collection(&mut self, collection_name: String) -> Result<()> {
        let collection = match self
            .qdrant
            .create_collection(
                CreateCollectionBuilder::new(collection_name.clone())
                    .vectors_config(VectorsConfigBuilder::default()),
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(err.into()),
        };
        self.all_collections.push(collection_name);
        Ok(())
    }
}
