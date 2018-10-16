use std::fs;
use std::error::Error;
use std::path::Path;
use std::io::prelude::*;

const ARGS_NUM: usize = 3;


pub struct Config {
    new_file: String,
    file_to_copy: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() != ARGS_NUM {
            return Err("number of arguments does not match required");
        }

        let file_to_copy = args[1].clone();
        let new_file = args[2].clone();

        Ok(Config { file_to_copy, new_file })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(conf.file_to_copy)?;
    let path = Path::new(&conf.new_file);

    let mut new_file = fs::File::create(&path)?;

    new_file.write_all(content.as_bytes())?;

    Ok(())
}
