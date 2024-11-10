use clap::Parser;
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;


// Estructura para los argumentos de la CLI
#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 12)]
    length: usize, // Longitud de la contraseña

    #[arg(short, long)]
    numbers: bool, // Incluir números

    #[arg(short, long)]
    symbols: bool, // Incluir símbolos
}

fn generate_password(length: usize, include_numbers: bool, include_symbols: bool) -> String {
    let mut rng = thread_rng();
    let mut password: Vec<char> = Vec::new();

    let alphabet: Vec<char> = if include_numbers && include_symbols {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()"
            .chars()
            .collect()
    } else if include_numbers {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .chars()
            .collect()
    } else {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect()
    };

    for _ in 0..length {
        password.push(*alphabet.choose(&mut rng).unwrap());
    }

    password.into_iter().collect()
}


fn main() {
    let args = Cli::parse(); // Parsear argumentos
    let password = generate_password(args.length, args.numbers, args.symbols);
    println!("Generated Password: {}", password);
}

