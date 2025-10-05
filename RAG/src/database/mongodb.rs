use std::{io::Result, ops::Deref};

use bson::Document;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Collection,
};

pub struct MongoClient {
    client: Client,
}

impl Deref for MongoClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl MongoClient {
    pub fn new() -> Result<MongoClient> {
        let host = dotenv::var("HOST").unwrap();
        let username = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let port = dotenv::var("PORT")
            .map(|elem| elem.parse::<u16>().unwrap())
            .ok();

        let client_options = ClientOptions::builder()
            .app_name("RAG".to_string())
            .hosts(vec![ServerAddress::Tcp { host, port }])
            .credential(Some(
                Credential::builder()
                    .username(username)
                    .password(password)
                    .build(),
            ))
            .build();
        let client = Client::with_options(client_options).unwrap();
        Ok(Self { client })
    }
}

pub trait UpsertCollection<T> {
    async fn upsert_one(&self, query: Document, upsert_doc: Document) -> Result<()>;
    async fn upsert_many(&self, query: Document, upsert_docs: &[T]) -> Result<()>;
}


// TODO 
impl<T: Sync + Send> UpsertCollection<T> for Collection<T> {
    async fn upsert_one(&self, query: Document, upsert_doc: Document) -> Result<()> {
        match self.find(query).await {
            Ok(res) => Ok(()),
            Err(err) => Ok(()),
        }
    }

    async fn upsert_many(&self, query: Document, upsert_docs: &[T]) -> Result<()> {
        todo!()
    }
}
