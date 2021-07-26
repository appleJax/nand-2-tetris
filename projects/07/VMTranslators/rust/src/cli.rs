use crate::vm_translator::Translator;
use std::{error::Error, fs};

pub struct Config {
    path: String,
}

impl Config {
    pub fn new<I>(mut args: I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let path = match args.next() {
            Some(p) => p,
            None => return Err("No file or directory given."),
        };

        Ok(Config { path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path.clone())?;

    let assembly_code = Translator::translate(contents);

    let output_file = config.path.replace(".vm", ".asm");
    fs::write(output_file, assembly_code)?;
    Ok(())
}
