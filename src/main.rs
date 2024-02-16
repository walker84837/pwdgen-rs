use anyhow::Result;
use clap::Parser;
use rand::{seq::SliceRandom, Rng};

/// A password generator, written in Rust.
#[derive(Parser)]
struct Args {
    #[arg(short, help = "The number of passwords to generate")]
    number: Option<usize>,
}

fn get_random_int(limit: usize) -> usize {
    let random_number: usize = rand::thread_rng().gen();
    random_number % limit
}

fn get_password() -> String {
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut password: Vec<char> = Vec::with_capacity(26);

    for _ in 0..18 {
        let random_index = get_random_int(26);
        password.push(lowercase_letters.chars().nth(random_index).unwrap());
    }

    password.push((get_random_int(26) as u8 + 65u8) as char);
    password.shuffle(&mut rand::thread_rng());

    password.push((get_random_int(10) as u8 + 48u8) as char);
    password.shuffle(&mut rand::thread_rng());

    password[6] = '-';
    password[13] = '-';

    password.into_iter().collect()
}

fn main() -> Result<()> {
    let cli = Args::parse();
    let quantity = cli.number.unwrap_or(1);

    for _ in 0..quantity {
        println!("{}", get_password());
    }

    Ok(())
}
