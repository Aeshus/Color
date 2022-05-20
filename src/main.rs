use std::env::args;

use crate::cli::Cli;
use crate::parser::Png;

mod cli;
mod parser;

fn main() {
    let cli = Cli::from(args());

    let png = Png::from(cli);

    println!("{}", png);
}
