use clap::Parser;
use serde::Deserialize;
use rand::seq::IndexedRandom;

use std::fs::File;
use std::path::Path;


#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    author: String,
}

#[derive(Deserialize, Debug)]
struct Quote {
    author: String,
    quotes: Vec<String>,
}


impl Quote {
    fn get_quote(target_author: String) -> Option<Quote> {
        let json_file_path = Path::new("./quotes.json");
        let file = File::open(json_file_path).expect("Failed to open file");
        let quotes: Vec<Quote> = serde_json::from_reader(file).expect("error while reading");

        for quote in quotes {
            if target_author.to_string() == quote.author.to_string() {
                return Some(quote);
            }
        }
        None
    }
}

fn main() {
    let args = Cli::parse();
    
    match Quote::get_quote(args.author) {
        Some(author_struct) => {
            println!("{:?} says this quotes: {:?}", author_struct.author, author_struct.quotes.choose(&mut rand::rng()).unwrap());
        },
        None => {
            println!("No aviable author spesified! ---");
        }
        
    }   
}
