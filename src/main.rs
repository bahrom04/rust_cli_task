mod error;
mod config;
mod test;

use crate::error::CustomError;
use crate::config::Cli;

use clap::Parser;
use serde::Deserialize;
use serde;
use rand::seq::IndexedRandom;

static BINARY_DATA: &str = include_str!("../data/quotes.json");

#[derive(Deserialize, Debug, PartialEq)]
struct Quote {
    author: String,
    quotes: Vec<String>,
}

impl Quote {
    fn new(author: String, quotes: Vec<String>) -> Self {
        Self {
            author, quotes
        }
    }

    fn from_json(path: &str) -> Result<Vec<Quote>, CustomError> {
        let quotes: Vec<Quote> = serde_json::from_str(path)?;
        
        Ok(quotes)
    }

    fn get_quote(target_quote: &str, quotes: Vec<Quote>) -> Option<Quote> {
        for quote in quotes {
            if target_quote == quote.author {
                return Some(quote);
            }
        }
        None
    }
}

fn main() {
    let args: Cli = Cli::parse();

    match Quote::from_json(BINARY_DATA) {
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

