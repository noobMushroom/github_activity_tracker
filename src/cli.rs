use crate::github::handle_request;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    user: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    User { username: String },
}

pub fn parse_args() {
    let cli = Cli::parse();
    if let Err(err) = handle_request(&cli.user) {
        println!("Error: {}", err);
    }
}
