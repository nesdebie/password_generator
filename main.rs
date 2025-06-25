use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Rust Password Generator");

    let length = {
        let input = read_input("Enter password length or press Enter for default (min 16): ").trim().to_string();

        match input.parse::<usize>() {
            Ok(len) if len >= 16 => len,
            _ => {
                println!("Using default length of 16.");
                16
            }
        }
    };

    let password = generate_password(length);
    println!("{}", password);
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim_end().to_string()
}

fn generate_password(length: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz\
                              ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                              0123456789\
                              !@#$%^&*()-_=+[]{};:,.<>/?"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect()
}

