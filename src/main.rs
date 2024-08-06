use anyhow::{bail, Result};
use clap::Parser;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

/// A lightning-fast password generator, written in Rust.
#[derive(Parser)]
struct Args {
    #[clap(short, default_value = "1", help = "The number of passwords")]
    number: usize,

    #[clap(
        short = 's',
        long = "sections",
        default_value = "3",
        help = "The number of sections in the password"
    )]
    sections: usize,
}

fn get_password(sections: usize) -> String {
    const SECTION_LENGTH: usize = 6;
    const LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    const MAX_LETTERS_INDEX: usize = LOWERCASE_LETTERS.len();

    let get_random_int = |limit: usize| -> usize {
        let random_number: usize = rand::thread_rng().gen();
        random_number % limit
    };

    let mut password_chars = Vec::new();

    // Add one random uppercase letter and one digit to the password
    password_chars.push((get_random_int(26) as u8 + 65u8) as char);
    password_chars.push((get_random_int(10) as u8 + 48u8) as char);

    // Fill the rest of the password with lowercase letters
    let total_length = sections * SECTION_LENGTH - 2;
    for _ in 0..total_length {
        let random_index = get_random_int(MAX_LETTERS_INDEX);
        password_chars.push(LOWERCASE_LETTERS.chars().nth(random_index).unwrap());
    }

    // Shuffle the characters to randomize their order
    password_chars.shuffle(&mut rand::thread_rng());

    // Split the password into sections
    let mut password_sections = Vec::new();
    for section in password_chars.chunks(SECTION_LENGTH) {
        password_sections.push(section.iter().collect::<String>());
    }

    // Join sections with hyphens
    password_sections.join("-")
}

fn main() -> Result<()> {
    let cli = Args::parse();

    if cli.sections == 0 {
        bail!("Number of sections can't be zero.");
    }

    (0..cli.number).into_par_iter().for_each(|_| {
        println!("{}", get_password(cli.sections));
    });

    Ok(())
}
