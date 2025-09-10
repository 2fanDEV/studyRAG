use actix::Message;

pub mod vector_embedding_model;

#[derive(Message)]
#[rtype(result = "Result<Vec<Vec<f32>>, std::io::Error>")]
pub struct EmbeddingMessages {
    pub full_text: Vec<SingleMessage>,
}

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
    fn process(&self, msg: EmbeddingMessages) -> Vec<Vec<f32>>;
}

pub fn split_by_context_size(
    full_text: String,
    ctx_size: usize,
    words_from_previous_sentence: usize,
) -> EmbeddingMessages {
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
    EmbeddingMessages {
        full_text: container,
    }
}

mod tests {

    #[cfg(test)]
    mod tests {
        use crate::model::bert_actors::split_by_context_size;

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

            let splitted_sentences = split_by_context_size(full_text.to_string(), 30, 10).full_text;
            for (index, sentence) in splitted_sentences.into_iter().enumerate() {
                let expected_result = expected_results[index];
                assert_eq!(sentence.text, expected_result.0);
                assert_eq!(sentence.amount_words, expected_result.1)
            }
        }
    }
}
