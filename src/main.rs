mod error;
mod config;

use crate::error::CustomError;
use crate::config::Cli;

use clap::Parser;
use serde::Deserialize;
use serde;
use rand::seq::IndexedRandom;
use std::env;
use std::fs::File;


#[derive(Deserialize, Debug)]
struct Quote {
    author: String,
    quotes: Vec<String>,
}

impl Quote {
    fn read_json(path: &str) -> Result<Vec<Quote>, CustomError> {
        let mut exe_path = env::current_exe()?;
        exe_path.pop();
        exe_path.push(path);
        // let json_file_path = Path::new(&exe_path);
        let json_file_path = exe_path.clone();

        if !json_file_path.exists() {
            return Err(CustomError::JsonNotFoundError(json_file_path));
        }

        let file = File::open(json_file_path)?;
        let quotes: Vec<Quote> = serde_json::from_reader(file)?;

        Ok(quotes)
    }

    fn get_quote(target_author: &str, quotes: Vec<Quote>) -> Option<Quote> {
        for quote in quotes {
            if target_author == quote.author {
                return Some(quote);
            }
        }
        None
    }
}

fn main() {
    let args = Cli::parse();
    let path = "quotes.json";

    match Quote::read_json(&path) {
        Ok(quotes) => {
            match Quote::get_quote(&args.author, quotes) {
                Some(quote) => {
                    println!("{:?} says this quotes: {:?}\n", quote.author, quote.quotes.choose(&mut rand::rng()).unwrap());
                }
                None => {
                    println!("todo not found quote error");
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
        
    }   
}
