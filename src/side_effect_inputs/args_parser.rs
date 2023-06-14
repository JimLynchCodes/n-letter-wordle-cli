use clap::Parser;

/// A wordle clone, played in the command line, with words of any length.
#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    ///  Number of letters in the word to guess.
    #[arg(default_value_t = 5)]
    pub letters_in_word: u8,

    ///  Number of guesses the user gets to make.
    #[arg(short, long, default_value_t = 6)]
    pub guesses: u8,

    ///  Print extra debug information.
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    ///  Display the currently installed version.
    #[arg(short, long)]
    pub version: bool, 
}

pub fn read_args() -> Cli {
    Cli::parse()
}
