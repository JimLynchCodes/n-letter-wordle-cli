use std::error::Error;

use reqwest::blocking::get;
use serde_json::{from_str, Value};

/// Returns a tuple of the part of speech and defintion for the given word.
pub fn get_part_of_speech_and_definition(word: &str) -> Result<(Value, bool), Box<dyn Error>> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = get(url)?.text()?;
    let serde_result = serde_json::from_str::<Value>(&response)?;

    match serde_result.get("title") {
        Some(title) => {
            if title == "No Definitions Found" {
                Ok((
                    from_str(r#"{ "message": "Huh, we couldn't find any definitions..." }"#)
                        .unwrap(),
                    false,
                ))
            } else {
                panic!("definition result: {}", title);
            }
        }
        None => {
            let meanings = serde_result[0].get("meanings").unwrap();
            Ok((meanings.to_owned(), true))
        }
    }

}

#[cfg(test)]
mod get_part_of_speech_and_definition_tests {

    use std::error::Error;

    use crate::side_effect_lib_wrappers::get_part_of_speech_and_definition::get_part_of_speech_and_definition;

    #[test]
    fn definition_for_hello() -> Result<(), Box<dyn Error>> {
        let expected = "[{\"antonyms\":[],\"definitions\":[{\"antonyms\":[],\"definition\":\"\\\"Hello!\\\" or an equivalent greeting.\",\"synonyms\":[]}],\"partOfSpeech\":\"noun\",\"synonyms\":[\"greeting\"]},{\"antonyms\":[],\"definitions\":[{\"antonyms\":[],\"definition\":\"To greet with \\\"hello\\\".\",\"synonyms\":[]}],\"partOfSpeech\":\"verb\",\"synonyms\":[]},{\"antonyms\":[\"bye\",\"goodbye\"],\"definitions\":[{\"antonyms\":[],\"definition\":\"A greeting (salutation) said when meeting someone or acknowledging someone’s arrival or presence.\",\"example\":\"Hello, everyone.\",\"synonyms\":[]},{\"antonyms\":[],\"definition\":\"A greeting used when answering the telephone.\",\"example\":\"Hello? How may I help you?\",\"synonyms\":[]},{\"antonyms\":[],\"definition\":\"A call for response if it is not clear if anyone is present or listening, or if a telephone conversation may have been disconnected.\",\"example\":\"Hello? Is anyone there?\",\"synonyms\":[]},{\"antonyms\":[],\"definition\":\"Used sarcastically to imply that the person addressed or referred to has done something the speaker or writer considers to be foolish.\",\"example\":\"You just tried to start your car with your cell phone. Hello?\",\"synonyms\":[]},{\"antonyms\":[],\"definition\":\"An expression of puzzlement or discovery.\",\"example\":\"Hello! What’s going on here?\",\"synonyms\":[]}],\"partOfSpeech\":\"interjection\",\"synonyms\":[]}]";

        // assert_eq!(
        //     get_part_of_speech_and_definition("hello")?.to_string(),
        //     expected
        // );

        Ok(())
    }

    //     #[test]
    //     fn invalid_words_return_false() -> Result<(), Box<dyn Error>> {
    //         assert_eq!(false, is_valid_english_word("notaword", ""));

    //         Ok(())
    //     }

    //     #[test]
    //     fn invalid_word_that_is_the_secret_word_returns_true() -> Result<(), Box<dyn Error>> {
    //         assert!(is_valid_english_word("notaword", "notaword"));

    //         Ok(())
    //     }
}
