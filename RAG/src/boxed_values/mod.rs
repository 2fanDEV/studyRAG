use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(String);

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Serialize)]
pub struct Count {
    pub count: u64
}



