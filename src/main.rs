use colored::Colorize;

mod crack;
mod decode;
mod encode;

mod caesor;

mod app;

use clap::Parser;

use crate::{crack::crack, decode::decode, encode::encode};

fn main() {
    let args = app::Cli::parse();

    match args.command {
        app::Commands::Encode(app::EncodeOptions { text, key }) => {
            if args.verbose {
                println!(
                    "Encode \"{}\" with the key \"{}\":",
                    &text.to_string().blue().bold(),
                    &key.to_string().yellow().bold()
                );
            }
            let encoded = encode(&text, &key);
            println!("{}", encoded.to_string().green().bold());
        }
        app::Commands::Decode(app::DecodeOptions { text, key }) => {
            if args.verbose {
                println!(
                    "Decoding \"{}\" with the key \"{}\"",
                    &text.to_string().blue().bold(),
                    &key.to_string().yellow().bold()
                );
            }
            let decoded = decode(&text, &key);
            println!("{}", decoded.green().bold());
        }
        app::Commands::Crack(app::CrackOptions { text, length }) => {
            if args.verbose {
                println!("Cracked \"{}\":", &text.to_string().blue().bold());
            }

            let cracked = crack(&text, &length);
            println!("{}", cracked.green().bold());
        }
    }
}
