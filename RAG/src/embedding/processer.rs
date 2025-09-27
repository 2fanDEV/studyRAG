use text_splitter::{ChunkConfig, ChunkSizer, TextSplitter};
use tokenizers::{Result, Tokenizer};

use crate::{
    boxed_values::Token,
    embedding::processer::TextProcesserUtil::percentage_from_start_of_token_chunk,
};

pub type TokenAndText = (String, Vec<Token>);

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

    pub fn process(&self, full_text: &str) -> Result<Vec<TokenAndText>> {
        let mut tokenized_chunks: Vec<TokenAndText> = vec![];
        let chunks = self.chunk_text(full_text);
        for chunk in chunks {
            let tokens = self.tokenize_text(&chunk)?;
            tokenized_chunks.push((chunk, tokens));
        }

        let end_index = tokenized_chunks.len() - 1;
        for (index, (_text, mut tokenized_chunk)) in tokenized_chunks.clone().into_iter().enumerate()
        {
            if index == end_index {
                break;
            }
            self.append_percentage_of_current_to_previous_tokens(
                &mut tokenized_chunk,
                &tokenized_chunks[index + 1].1,
                0.2,
            );
        }

        Ok(tokenized_chunks)
    }

    fn append_percentage_of_current_to_previous_tokens(
        &self,
        previous: &mut Vec<Token>,
        current: &[Token],
        percentage: f32,
    ) {
        let percentage_splitted_chunk = percentage_from_start_of_token_chunk(current, percentage);
        previous.extend_from_slice(percentage_splitted_chunk);
    }

    fn chunk_text(&self, full_text: &str) -> Vec<String> {
        self.splitter
            .chunks(full_text)
            .map(|chunk| chunk.to_string())
            .collect::<Vec<_>>()
    }

    fn tokenize_text(&self, chunk_text: &str) -> Result<Vec<Token>> {
        let encoding = self.tokenizer.encode(chunk_text, true)?;
        let tokens = encoding
            .get_tokens()
            .into_iter()
            .map(|text| Token(text.to_string()))
            .collect::<Vec<_>>();
        Ok(tokens)
    }
}

#[allow(non_snake_case)]
mod TextProcesserUtil {
    use crate::boxed_values::Token;

    pub fn percentage_from_start_of_token_chunk(chunk: &[Token], percentage: f32) -> &[Token] {
        let index_of_percentage = (((chunk.len() as f32) * percentage).ceil()) as usize;
        &chunk[0..index_of_percentage]
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            boxed_values::Token,
            embedding::processer::TextProcesserUtil::percentage_from_start_of_token_chunk,
        };

        #[test]
        fn percentage_from_start_of_chunk_test() {
            let tokens = ["1", "2", "3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens = ["1"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens1 = ["1", "2"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens2 = ["1", "2", "3"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens3 = ["1", "2", "3", "4"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens4 = ["1", "2", "3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            assert_eq!(
                expected_tokens,
                percentage_from_start_of_token_chunk(&tokens, 0.2)
            );
            assert_eq!(
                expected_tokens1,
                percentage_from_start_of_token_chunk(&tokens, 0.4)
            );
            assert_eq!(
                expected_tokens2,
                percentage_from_start_of_token_chunk(&tokens, 0.6)
            );
            assert_eq!(
                expected_tokens3,
                percentage_from_start_of_token_chunk(&tokens, 0.8)
            );
            assert_eq!(
                expected_tokens4,
                percentage_from_start_of_token_chunk(&tokens, 1.0)
            );
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod TextProcessorTests {

    use crate::boxed_values::Token;
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
        let chunk1 = "My name is Tufan.";
        let chunk2 = "I am a software developer";
        let mut tokenized_chunk1 = processor.tokenize_text(&chunk1).unwrap();
        let tokenized_chunk2 = processor.tokenize_text(&chunk2).unwrap();
        let expected = [
            "[CLS]", "my", "name", "is", "tu", "##fan", ".", "[SEP]", "[CLS]", "i",
        ]
        .iter()
        .map(|t| Token(t.to_string()))
        .collect::<Vec<_>>();
        processor.append_percentage_of_current_to_previous_tokens(
            &mut tokenized_chunk1,
            &tokenized_chunk2,
            0.2,
        );
        assert_eq!(expected, tokenized_chunk1);
    }

    #[test]
    fn tokenize_text_test() {
        let (processor, text) = before_each(512);
        let tokenized_text = processor.tokenize_text(&text).unwrap();
        let expected_tokens = [
            "[CLS]",
            "hi",
            "my",
            "name",
            "is",
            "2",
            "##fan",
            ".",
            "i",
            "am",
            "a",
            "software",
            "developer",
            ".",
            "i",
            "love",
            "music",
            "a",
            "lot",
            ".",
            "[SEP]",
        ]
        .into_iter()
        .map(|elem| Token(elem.to_string()))
        .collect::<Vec<Token>>();
        assert_eq!(expected_tokens, tokenized_text);
    }
}
