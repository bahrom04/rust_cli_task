mod config;
mod error;
mod test;

use crate::config::Cli;
use crate::error::CustomError;

use clap::Parser;
use rand::seq::IndexedRandom;
use serde;
use serde::Deserialize;

static BINARY_DATA: &str = include_str!("../data/quotes.json");

#[derive(Deserialize, Debug, PartialEq)]
struct Quote {
    author: String,
    quotes: Vec<String>,
}

impl Quote {
    fn new() -> Self {
        let author: String = String::new();
        let quotes: Vec<String> = Vec::new();
        Self { author, quotes }
    }

    fn get_quote(self, target_quote: &str, quotes: Vec<Quote>) -> Option<Quote> {
        for quote in quotes {
            if target_quote == quote.author {
                return Some(quote);
            }
        }
        None
    }

    fn from_json(&self, path: &str) -> Result<Vec<Quote>, CustomError> {
        serde_json::from_str(path).map_err(CustomError::JsonParse)
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let quote: Quote = Quote::new();

    // let () = quote
    //     .from_json(BINARY_DATA)
    //     .map_or_else(
    //         |err| {
    //             println!("{}", err);
    //             None
    //         },
    //         |quotes| quote.get_quote(&args.author, quotes),
    //     )
    //     .map_or_else(
    //         || {
    //             println!("todo not found quote error");
    //         },
    //         |op| {
    //             println!(
    //                 "{:?} says this quotes: {:?}\n",
    //                 op.author,
    //                 op.quotes.choose(&mut rand::rng()).unwrap()
    //             );
    //         },
    //     );

    match quote.from_json(BINARY_DATA) {
        Ok(quotes) => match quote.get_quote(&args.author, quotes) {
            Some(quote) => {
                println!(
                    "{:?} says this quotes: {:?}\n",
                    quote.author,
                    quote.quotes.choose(&mut rand::rng()).unwrap()
                );
            }
            None => {
                println!("todo not found quote error");
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
