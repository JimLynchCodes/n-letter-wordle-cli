use std::error::Error;

use indexmap::IndexMap;
use zspell::Dictionary;

use crate::data_elements::data::GuessState;
use crate::pure_functions::guess_colorizer::build_colored_guess_string;
use crate::pure_functions::guess_scorer::score_guess;
use crate::side_effect_inputs::text_input_handler::get_guess_from_user;
use crate::side_effect_outputs::console_printer::{
    print_guesses_left, print_prev_scored_guesses, print_you_lose, print_you_win,
};
use crate::side_effect_outputs::render_config::build_text_input_render_config;

pub fn prompt_for_guess(
    remaining_guesses: u8,
    secret_word: String,
    letters_in_word: usize,
    prev_letters_guessed: IndexMap<String, GuessState>,
    guesses_already_made: u8,
    mut scored_guesses: Vec<String>,
    debug_mode: bool,
    dict: Dictionary,
) -> Result<(), Box<dyn Error>> {
    inquire::set_global_render_config(build_text_input_render_config());

    print_guesses_left(remaining_guesses);

    let guess =
        get_guess_from_user(letters_in_word, secret_word.clone(), &prev_letters_guessed).unwrap();

    let (new_letters_guessed, guessed_correctly, scored_letters) = score_guess(
        guess.clone(),
        secret_word.clone(),
        letters_in_word,
        prev_letters_guessed,
        true, // debug_mode,
    )
    .unwrap();

    let scored_guess_str = build_colored_guess_string(scored_letters).unwrap();
    scored_guesses.push(format!("Guess #{guesses_already_made}) {scored_guess_str}"));

    print_prev_scored_guesses(&scored_guesses);

    if guessed_correctly {
        print_you_win(guesses_already_made, &secret_word)?;
    } else if remaining_guesses == 1 {
        print_you_lose(&secret_word)?;
    } else {
        prompt_for_guess(
            remaining_guesses - 1,
            secret_word,
            letters_in_word,
            new_letters_guessed,
            guesses_already_made + 1,
            scored_guesses,
            debug_mode,
            dict,
        )?;
    }

    Ok(())
}
