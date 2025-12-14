use anyhow::{Result, bail};
use clap::Parser;
use rand::{Rng, seq::SliceRandom};
use rayon::prelude::*;

const LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const MAX_LETTERS_INDEX: usize = LOWERCASE_LETTERS.len();

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

    #[clap(
        short = 'l',
        long = "length",
        default_value = "6",
        help = "The length of each section"
    )]
    section_length: usize,
}

fn get_password(sections: usize, section_length: usize) -> String {
    let get_random_int = |limit: usize| -> usize { rand::rng().random_range(0..limit) };

    let mut password_chars = Vec::new();

    // Add one random uppercase letter and one digit to the password
    password_chars.push((get_random_int(26) as u8 + 65u8) as char);
    password_chars.push((get_random_int(10) as u8 + 48u8) as char);

    // Fill the rest of the password with lowercase letters
    let total_length = sections * section_length - 2;
    for _ in 0..total_length {
        let random_index = get_random_int(MAX_LETTERS_INDEX);
        password_chars.push(LOWERCASE_LETTERS.chars().nth(random_index).unwrap());
    }

    // Shuffle the characters to randomize their order
    password_chars.shuffle(&mut rand::rng());

    // Split the password into sections
    let mut password_sections = Vec::new();
    for section in password_chars.chunks(section_length) {
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

    if cli.section_length == 0 {
        bail!("Section length can't be zero.");
    }

    (0..cli.number).into_par_iter().for_each(|_| {
        println!("{}", get_password(cli.sections, cli.section_length));
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_dash_placement() {
        let sections = 3;
        let section_length = 5;
        let password = get_password(sections, section_length);
        let parts: Vec<&str> = password.split('-').collect();

        assert_eq!(parts.len(), sections);
        for part in parts {
            assert_eq!(part.len(), section_length);
        }
    }

    #[test]
    fn test_password_has_uppercase_letter() {
        let password = get_password(3, 6);
        assert!(password.chars().any(|c| c.is_uppercase()));
    }

    #[test]
    fn test_password_has_number() {
        let password = get_password(3, 6);
        assert!(password.chars().any(|c| c.is_digit(10)));
    }
}
