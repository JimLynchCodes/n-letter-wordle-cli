use std::error::Error;

use indexmap::IndexMap;

use crate::data_elements::data::GuessState;

pub fn score_guess(
    guess: String,
    secret_word: String,
    letters_in_word: usize,
    mut prev_letters_guessed: IndexMap<String, GuessState>,
    debug_mode: bool,
) -> Result<
    (
        IndexMap<String, GuessState>,
        bool,
        Vec<(String, GuessState)>,
    ),
    Box<dyn Error>,
> {
    let mut scored_letters: Vec<(String, GuessState)> = vec![];

    for i in 0..letters_in_word {
        let guess_char = guess.chars().nth(i.into()).unwrap();
        let secret_char = secret_word.chars().nth(i.into()).unwrap();

        // if debug_mode {
        //     print!("\nComparing char {i} of guess {guess_char} to {secret_char}");
        // }

        if guess_char == secret_char {
            if debug_mode {
                // print!(", green");
            }

            prev_letters_guessed
                .insert(
                    guess_char.to_string(),
                    GuessState::InWordFoundLocation(i as u8),
                )
                .unwrap();

            scored_letters.push((
                guess_char.to_string(),
                GuessState::InWordFoundLocation(i as u8),
            ))
        } else if secret_word.contains(guess_char) {
            if debug_mode {
                // print!(", yellow");
            }
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::InWordUnknownLocation)
                .unwrap();

            scored_letters.push((guess_char.to_string(), GuessState::InWordUnknownLocation))
        } else {
            if debug_mode {
                // print!(", black");
            }
            prev_letters_guessed
                .insert(guess_char.to_string(), GuessState::GuessedNotInWord)
                .unwrap();
            scored_letters.push((guess_char.to_string(), GuessState::GuessedNotInWord))
        }
    }

    // if debug_mode {
    //     print!("\n");
    // }

    Ok((prev_letters_guessed, guess == secret_word, scored_letters))
}
