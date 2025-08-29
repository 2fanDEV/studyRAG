use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::model::IntoDocument;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(String);

impl Deref for Id {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoDocument for Id {}
