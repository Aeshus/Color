use std::env::args;

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::from(args());

    println!("{:?}", cli)
}
