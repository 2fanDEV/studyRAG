use actix::{Actor, Handler, Message, MessageResult, SyncContext};
use rust_bert::pipelines::keywords_extraction::{Keyword, KeywordExtractionModel};

pub struct ActorTest {
    pub model: KeywordExtractionModel<'static>,
}

impl Actor for ActorTest {
    type Context = SyncContext<Self>;
}

pub struct ExtractionText(pub String);

impl Message for ExtractionText {
    type Result = Vec<Vec<Keyword>>;
}

impl Handler<ExtractionText> for ActorTest {
    type Result = MessageResult<ExtractionText>;

    fn handle(&mut self, msg: ExtractionText, ctx: &mut Self::Context) -> Self::Result {
        let predict = self.model.predict(&[msg.0]).unwrap();
        MessageResult(predict)
    }
}
