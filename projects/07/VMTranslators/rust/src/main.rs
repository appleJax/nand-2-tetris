mod cli;
mod code_gen;
mod file_reader;
mod parser;
mod vm_translator;

use std::{env, process};

fn main() {
    let config = cli::Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    if let Err(err) = cli::run(config) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
