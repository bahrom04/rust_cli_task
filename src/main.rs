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
use std::path::PathBuf;


#[derive(Deserialize, Debug, PartialEq)]
struct Quote {
    author: String,
    quotes: Vec<String>,
}

impl Quote {
    // fn new(author: String, quotes: Vec<String>) -> Self {
    //     Self {
    //         author, quotes
    //     }
    // }

    fn read_json(path: PathBuf) -> Result<Vec<Quote>, CustomError> {
        let mut exe_path: PathBuf = env::current_exe()?;
        exe_path.pop();
        exe_path.push(path);
        // let json_file_path = Path::new(&exe_path);
        let json_file_path: PathBuf = exe_path.clone();

        if !json_file_path.exists() {
            return Err(CustomError::JsonNotFoundError(json_file_path));
        }

        let file = File::open(json_file_path)?;
        let quotes: Vec<Quote> = serde_json::from_reader(file)?;

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
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("./data/quotes.json");

    match Quote::read_json(path) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_json_method(){
        let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("./data/test.json");
        let quotes: Vec<Quote> = Quote::read_json(path).unwrap();

        let test_quote_data_1: Quote = Quote{
            author: "bahrom04".to_string(), 
            quotes: vec!["oʻzbekchasi yoʻq ekan".to_string(), "asahi oʻrnatsammikan?".to_string()] 
        };
        let test_quote_data_2: Quote = Quote{
            author: "orzklv".to_string(), 
            quotes: vec!["cooked".to_string(), "koʻkaldosh".to_string()] 
        };

        assert_eq!(quotes, vec![test_quote_data_1, test_quote_data_2])

    }
}