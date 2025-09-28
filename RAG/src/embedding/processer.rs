use log::debug;
use text_splitter::{ChunkConfig, ChunkSizer, TextSplitter};
use tokenizers::{Result, Tokenizer};

use crate::{
    embedding::processer::TextProcesserUtil::percentage_from_start_of_string,
};


#[derive(Debug)]
pub struct TextProcessor {
    tokenizer: Tokenizer,
    splitter: TextSplitter<Tokenizer>,
}

impl TextProcessor {
    pub fn new(chunk_size: usize) -> Result<Self> {
        let tokenizer = Tokenizer::from_pretrained("bert-base-uncased", None)?;
        Ok(TextProcessor {
            tokenizer: tokenizer.clone(),
            splitter: TextSplitter::new(ChunkConfig::new(chunk_size).with_sizer(tokenizer)),
        })
    }

    pub fn process(&self, full_text: &str) -> Result<Vec<String>> {
        let chunks = self.chunk_text(full_text);
        Ok(chunks)
    }

    fn append_percentage_of_current_to_previous_tokens(
        &self,
        previous: String,
        current: String,
        percentage: f32,
    ) -> String {
        let percentage_splitted_chunk = percentage_from_start_of_string(current, percentage);
        previous + &percentage_splitted_chunk
    }

    fn chunk_text(&self, full_text: &str) -> Vec<String> {
        self.splitter
            .chunks(full_text)
            .map(|chunk| chunk.to_string())
            .collect::<Vec<_>>()
    }
}

#[allow(non_snake_case)]
mod TextProcesserUtil {

    pub fn percentage_from_start_of_string(chunk: String, percentage: f32) -> String {
        let index_of_percentage = (((chunk.len() as f32) * percentage).ceil()) as usize;
        chunk[0..index_of_percentage].to_string()
    }

    #[cfg(test)]
    mod tests {
        use crate::embedding::processer::TextProcesserUtil::percentage_from_start_of_string;


        #[test]
        fn percentage_from_start_of_chunk_test() {
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod TextProcessorTests {

    use crate::embedding::processer::TextProcessor;

    type TestType = (TextProcessor, String);
    fn before_each(chunk_size: usize) -> TestType {
        let text_processor = TextProcessor::new(chunk_size).unwrap();
        (
            text_processor,
            String::from("Hi my name is 2fan. I am a software developer. I love music a lot."),
        )
    }

    #[test]
    fn append_percentage_of_current_to_previous_tokens_test() {
        let (processor, test) = before_each(10);
        let chunk1 = "My name is Tufan.".to_string();
        let chunk2 = "I am a software developer".to_string();
        let expected = "My name is Tufan. I am".to_string() ;
        processor.append_percentage_of_current_to_previous_tokens(
            chunk1.clone(),
            chunk2,
            0.2,
        );
        assert_eq!(expected, chunk1);
    }
}
