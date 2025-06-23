use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(required = true)]
    pub author: String,
    // quote_path: String,
}