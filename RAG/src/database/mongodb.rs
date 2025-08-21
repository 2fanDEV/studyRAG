use std::{
    io::{self, Result},
    ops::Deref,
    u16,
};

use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client,
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
