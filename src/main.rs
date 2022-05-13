use std::env;

use crate::cli::Cli;

//use crate::parser::Png;

mod cli;
mod error;
mod parser;

fn main() {
    let cli = Cli::from(env::args());

    println!("{:?}", cli)
}
