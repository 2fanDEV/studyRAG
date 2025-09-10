
use actix::{dev::MessageResponse, Actor, Context, Handler, Message};

use crate::model::bert_actors::{EmbeddingMessages, EmbeddingModel};

pub mod bert_actors;


pub struct EmbeddingActor {
    model: Box<dyn EmbeddingModel>,
}

impl Handler<EmbeddingMessages> for EmbeddingActor
{
    type Result = Result<Vec<Vec<f32>>, std::io::Error>;
    fn handle(&mut self, msg: EmbeddingMessages, ctx: &mut Self::Context) -> Self::Result {
        Ok(self.model.process(msg))
    }
}

impl Actor for EmbeddingActor {
    type Context = Context<Self>;
}
