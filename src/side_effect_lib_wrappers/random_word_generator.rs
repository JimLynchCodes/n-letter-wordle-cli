use std::error::Error;

use zspell::Dictionary;

use super::english_word_validator::is_valid_english_word;

pub fn generate_random_word(letters: u8) -> Result<String, Box<dyn Error>> {
    
    let generated_word = random_word::gen_len(letters.into()).unwrap_or("");

    // if let Some(word) = generated_word {
        
    //     is_valid_english_word(word, "");

    //     Ok()

    // }
    // else {
    //     Err("")
    // }
    if !is_valid_english_word(generated_word, "") {
        generate_random_word(letters)
    }
    else {
        Ok(generated_word.to_string())
    }

    // Ok().unwrap().to_string())
}
