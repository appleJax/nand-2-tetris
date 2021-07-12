use crate::assembler::Assembler;
use std::{error::Error, fs};

pub struct Config {
    filepath: String,
}

impl Config {
    pub fn new<I>(mut args: I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let filepath = match args.next() {
            Some(f) => f,
            None => return Err("Did not receive a file path."),
        };

        Ok(Config { filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath.clone())?;

    let assembly_code = Assembler::assemble(contents);

    let output_file = config.filepath.replace("hack", "asm");
    fs::write(output_file, assembly_code)?;
    Ok(())
}
