use crate::file_reader::FileReader;
use crate::vm_translator::Translator;
use std::{error::Error, fs, path::Path};

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
    let file_data = FileReader::process(&Path::new(&config.path))?;

    let mut translator = Translator::new();

    for (filename, contents) in file_data.files {
        translator.translate(filename, contents);
    }

    let assembly_code = translator.output();

    fs::write(file_data.output_filename, assembly_code)?;
    Ok(())
}
