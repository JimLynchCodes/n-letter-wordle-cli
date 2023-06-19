use zspell::{DictBuilder, Dictionary};

/// Returns true if the input word is a valid english word, otherwise false
/// Note: the secret word is always considered a word- even if it's not in the dictionary we are using for lookup
pub fn is_valid_english_word(word: &str, secret_word: &str) -> bool {
    let aff_content =
        std::fs::read_to_string("data/english-dictionary.aff").expect("failed to load config file");

    let dic_content = std::fs::read_to_string("data/english-dictionary.dic")
        .expect("failed to load wordlist file");

    let dict: Dictionary = DictBuilder::new()
        .config_str(&aff_content)
        .dict_str(&dic_content)
        .build()
        .expect("failed to build dictionary!");

    dict.check(word) || word == secret_word
}

#[cfg(test)]
mod is_valid_english_word_tests {

    use std::error::Error;

    use crate::side_effect_lib_wrappers::english_word_validator::is_valid_english_word;

    #[test]
    fn valid_word_return_true() -> Result<(), Box<dyn Error>> {
        assert!(is_valid_english_word("the", ""));
        assert!(is_valid_english_word("the", "the"));
        assert!(is_valid_english_word("a", ""));
        assert!(is_valid_english_word("disestablishment", ""));

        Ok(())
    }

    #[test]
    fn invalid_words_return_false() -> Result<(), Box<dyn Error>> {
        assert_eq!(false, is_valid_english_word("notaword", ""));

        Ok(())
    }

    #[test]
    fn invalid_word_that_is_the_secret_word_returns_true() -> Result<(), Box<dyn Error>> {
        assert!(is_valid_english_word("notaword", "notaword"));

        Ok(())
    }
}
