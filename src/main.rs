use std::fs;

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    /// The secret used to sign and verify the JWT
    secret: String,
    #[command(subcommand)]
    op: Ops,
}

#[derive(Subcommand)]
enum Ops {
    /// Generate a JWT
    Gen,
    /// Verify a JWT
    Verify,
}

fn main() {
    let cli = Cli::parse();
    match cli.op {
        Ops::Gen => generate_jwt(&cli.secret),
        Ops::Verify => verify_jwt(&cli.secret),
    }
    // println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug)]
struct Cliams {
    sub: String,
    exp: i64,
    iss: String,
}

fn generate_jwt(secret: &str) {
    println!("Generating...");
    let exp = (Utc::now() + Duration::days(1)).timestamp();
    let token = encode(
        &Header::new(Algorithm::HS256),
        &Cliams {
            sub: "test".to_string(),
            exp,
            iss: "test".to_string(),
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    fs::write("token.txt", token).unwrap();
    println!("Done!");
}

fn verify_jwt(secret: &str) {
    println!("Verifying...");
    let token_string = fs::read_to_string("token.txt").unwrap();
    let token = decode::<Cliams>(
        &token_string,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .unwrap();
    println!("{:?}", token.claims);
    println!("Done!");
}
