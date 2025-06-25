use rand::Rng;
use std::io;

fn main() {
    println!("Rust Password Generator");

    let length = loop {
        let input = read_input("Enter password length (min. 16): ").trim().to_string();

        match input.parse::<usize>() {
            Ok(len) if len >= 16 => break len,
            _ => {
                println!("Invalid input. Please enter a number >= 16.");
                continue;
            }
        }
    };

    let password = generate_password(length);
    println!("Generated password: {}", password);
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn generate_password(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyz\
                   ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                   0123456789\
                   !@#$%^&*()-_=+[]{};:,.<>/?";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
