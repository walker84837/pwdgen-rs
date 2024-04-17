use anyhow::Result;
use clap::Parser;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

/// A password generator, written in Rust.
#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "1", help = "The number of passwords")]
    number: usize,
}

fn get_password() -> String {
    const MAX_LENGTH: usize = 26;
    const LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

    let get_random_int = |limit: usize| -> usize {
        let random_number: usize = rand::thread_rng().gen();
        random_number % limit
    };

    let mut password: Vec<char> = Vec::with_capacity(MAX_LENGTH);

    for _ in 0..18 {
        let random_index = get_random_int(MAX_LENGTH);
        password.push(LOWERCASE_LETTERS.chars().nth(random_index).unwrap());
    }

    password.push((get_random_int(MAX_LENGTH) as u8 + 65u8) as char);
    password.push((get_random_int(10) as u8 + 48u8) as char);
    password.shuffle(&mut rand::thread_rng());

    password[6] = '-';
    password[13] = '-';

    password.into_iter().collect()
}

fn main() -> Result<()> {
    let cli = Args::parse();

    (0..cli.number).into_par_iter().for_each(|_| {
        println!("{}", get_password());
    });

    Ok(())
}
