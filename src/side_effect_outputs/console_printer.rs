use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use serde_json::Value;

use crate::side_effect_lib_wrappers::get_part_of_speech_and_definition::get_part_of_speech_and_definition;

pub fn print_initial_message() {
    println!("\nLet's play a word guessing game!");
    sleep_for_cool_ux();
}

pub fn print_im_thinking_of_a_word(letters: u8) {
    println!("\nI'm thinking of a random {letters} letter english word...");
    sleep_for_cool_ux();
}

pub fn print_guesses_left(remaining_guesses: u8) {
    println!("\nYou have {remaining_guesses} guesses remaining!\n");
    sleep_for_cool_ux();
}

pub fn print_prev_scored_guesses(scored_guesses: &Vec<String>) {
    println!("Your Guesses:");
    for prev_scored_guess in scored_guesses {
        println!("{prev_scored_guess}");
    }
    sleep_for_cool_ux();
}

pub fn print_you_win(guesses: u8, secret_word: &str) -> Result<(), Box<dyn Error>> {
    println!("\nYou got it!! The word was \"{}\"", secret_word);
    sleep_for_cool_ux();
    print_definitions(secret_word)?;
    println!("\nYou won in {guesses} guesses!\n");
    Ok(())
}

pub fn print_you_lose(secret_word: &str) -> Result<(), Box<dyn Error>> {
    println!("\nYou ran out of guesses! Better luck next time.");
    sleep_for_cool_ux();
    println!("\nThe word I was thinking of was: {}", secret_word);
    sleep_for_cool_ux();
    print_definitions(secret_word)?;
    Ok(())
}

pub fn print_definitions(secret_word: &str) -> Result<(), Box<dyn Error>> {
    // println!("\nYou ran out of guesses! Better luck next time.");
    // sleep_for_cool_ux();
    // print_definitions(secret_word);
    // sleep_for_cool_ux();
    println!("\nDefinitions for {}:\n", secret_word);
    // sleep_for_cool_ux();

    let meanings = get_part_of_speech_and_definition(secret_word)?;

    let meanings_arr: &Vec<Value> = meanings.as_array().unwrap();

    for (index, meaning) in meanings_arr.iter().enumerate() {
        println!(
            "{}) {}, {}\n",
            index + 1,
            meaning.get("partOfSpeech").unwrap(),
            meaning.get("definitions").unwrap()[0]
                .get("definition")
                .unwrap()
        );
        sleep_for_cool_ux();
    }

    Ok(())
}

pub fn print_deciding_if_valid_english_word(word: &str) {
    println!(
        "{}",
        format!("\n\nDeciding if {word} is a valid english word...\n")
    );
    sleep_for_cool_ux();
}

const MILLIS_TO_SLEEP: u64 = 650;

fn sleep_for_cool_ux() {
    sleep(Duration::from_millis(MILLIS_TO_SLEEP));
}
