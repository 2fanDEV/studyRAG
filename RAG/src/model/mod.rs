use std::marker::PhantomData;

use actix::Message;
use rust_bert::pipelines::keywords_extraction::Keyword;
use tokenizers::{Result, Tokenizer};

use crate::boxed_values::Token;

pub mod bert_actors;

pub trait ResultMarker {}

pub struct BertRequest<ResType>
where
    ResType: ResultMarker,
{
    pub full_text: Vec<SingleMessage>,

    pub _data: PhantomData<ResType>,
}

impl ResultMarker for Vec<Vec<f32>> {}
impl Message for BertRequest<Vec<Vec<f32>>> {
    type Result = Vec<Vec<f32>>;
}

impl ResultMarker for Vec<Vec<Keyword>> {}
impl Message for BertRequest<Vec<Vec<Keyword>>> {
    type Result = Vec<Vec<Keyword>>;
}

pub type EmbeddingMessagesRequest = BertRequest<Vec<Vec<f32>>>;
pub type ExtractionMessageRequest = BertRequest<Vec<Vec<Keyword>>>;

#[derive(Debug)]
pub struct SingleMessage {
    pub text: String,
    pub amount_words: usize,
}

impl AsRef<str> for SingleMessage {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

pub trait EmbeddingModel {
    fn process(&self, msg: EmbeddingMessagesRequest) -> Vec<Vec<f32>>;
}

pub trait ExtractionModel {
    fn process(&self, msg: ExtractionMessageRequest) -> Vec<Vec<Keyword>>;
}

/*
 * TODO: use ctx_size instead of splitting sentence by sentence
 * or split up sentence into two, if sentence.len > ctx_size
 */
pub fn split_by_context_size<T>(
    full_text: String,
    ctx_size: usize,
    words_from_previous_sentence: usize,
) -> BertRequest<T>
where
    T: ResultMarker,
{
    let mut container = vec![];
    let sentences = full_text
        .split(".")
        .map(|stri| stri.to_string())
        .collect::<Vec<String>>();
    for (index, mut sentence) in sentences.clone().into_iter().enumerate() {
        if sentence.eq("") {
            continue;
        }
        if index > 0 {
            let mut previous_sentence = sentences[index - 1].split_whitespace().collect::<Vec<_>>();
            let word_len = previous_sentence.len();
            if word_len > 10 {
                previous_sentence =
                    previous_sentence[(word_len - words_from_previous_sentence)..word_len].to_vec();
            }

            let sentence_with_partprevious = previous_sentence.join(" ") + ".";
            sentence = sentence_with_partprevious + &sentence;
        }

        sentence += ".";
        let length = sentence.split_whitespace().count();
        container.push(SingleMessage {
            text: sentence.to_string(),
            amount_words: length,
        })
    }
    BertRequest {
        full_text: container,
        _data: std::marker::PhantomData,
    }
}

pub fn chunk_tokens_by_ctx_size(tokens: Vec<Token>, chunk_size: usize) -> Vec<Vec<Token>> {
    let percentage_of_previous_chunk = 0.2 as f32;
    let mut chunked_tokens = tokens
        .chunks(chunk_size)
        .map(|slice| slice.to_vec())
        .collect::<Vec<_>>();
    for (index, chunk) in chunked_tokens.clone().into_iter().enumerate() {
        let mut previous_chunk = match chunked_tokens.get(index - 1) {
            Some(chunk) => {
                percentage_from_end_of_chunk(chunk.to_vec(), percentage_of_previous_chunk)
            }
            None => continue,
        };
        previous_chunk.extend_from_slice(&chunk);
        chunked_tokens[index] = previous_chunk;
    }
    chunked_tokens
}

pub fn percentage_from_end_of_chunk(chunk: Vec<Token>, percentage: f32) -> Vec<Token> {
    let index_of_percentage = ((chunk.len() as f32) * percentage) as usize;
    chunk[index_of_percentage..chunk.len() - 1].to_vec()
}

pub fn tokenize_text(full_text: &str) -> Result<Vec<Token>> {
    let tokenizer = Tokenizer::from_pretrained("bert-base-uncased", None)?;
    let encoding = tokenizer.encode(full_text, false)?;
    let tokens = encoding
        .get_tokens()
        .into_iter()
        .map(|text| Token(text.to_string()))
        .collect::<Vec<_>>();
    Ok(tokens)
}

mod tests {

    #[cfg(test)]
    mod tests {
        use crate::boxed_values::Token;
        use crate::model::percentage_from_end_of_chunk;
        use crate::model::split_by_context_size;
        use crate::model::tokenize_text;

        #[test]
        fn percentage_from_end_of_chunk_test() {
            let tokens = ["1", "2", "3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens1 = ["4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens2 = ["3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens3 = ["2", "3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            let expected_tokens4 = ["1", "2", "3", "4", "5"]
                .map(|elem| Token(elem.to_string()))
                .into_iter()
                .collect::<Vec<_>>();
            assert_eq!(percentage_from_end_of_chunk(tokens, 0.2), expected_tokens1);
        }

        #[test]
        fn tokenize_text_tes() {
            let full_text = "Hey, this text is going to be tokenized";
            let tokenized_text = tokenize_text(full_text).unwrap();
            let expected_tokens = [
                "hey", ",", "this", "text", "is", "going", "to", "be", "token", "##ized",
            ]
            .into_iter()
            .map(|elem| Token(elem.to_string()))
            .collect::<Vec<Token>>();
            assert_eq!(tokenized_text, expected_tokens);
        }

        #[test]
        fn split_by_context_size_test() {
            let full_text = "My name is 2fan. I am a software developer. Today I was at home and worked at my job for around 9 hours because there was an interesting bug. That bug persisted and accompanied me for around 3 hours as I found out what the problem was. Im just glad that there was no catastrophic issues.";
            let expected_results: Vec<(&str, usize)> = vec![
                ("My name is 2fan.", 4),
                ("My name is 2fan. I am a software developer.", 9),
                ("I am a software developer. Today I was at home and worked at my job for around 9 hours because there was an interesting bug.", 25),
                ("for around 9 hours because there was an interesting bug. That bug persisted and accompanied me for around 3 hours as I found out what the problem was.", 28),
                ("3 hours as I found out what the problem was. Im just glad that there was no catastrophic issues.", 19)
            ];

            let splitted_sentences =
                split_by_context_size::<Vec<Vec<f32>>>(full_text.to_string(), 30, 10).full_text;
            for (index, sentence) in splitted_sentences.into_iter().enumerate() {
                let expected_result = expected_results[index];
                assert_eq!(sentence.text, expected_result.0);
                assert_eq!(sentence.amount_words, expected_result.1)
            }
        }
    }
}
