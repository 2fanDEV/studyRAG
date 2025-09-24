use std::marker::PhantomData;

use actix::Message;
use rust_bert::pipelines::keywords_extraction::Keyword;

pub mod bert_actors;

pub trait ResultMarker {}

pub struct BertRequest<ResType>
where
    ResType: ResultMarker,
{
    full_text: Vec<SingleMessage>,

    _data: PhantomData<ResType>,
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

mod tests {

    #[cfg(test)]
    mod tests {
        use crate::model::split_by_context_size;

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
