use std::fs;
use std::error::Error;

const ARGS_NUM: usize = 2;


pub struct Config {
    file: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() != ARGS_NUM {
            return Err("number of arguments does not match required");
        }

        let file = args[1].clone();

        Ok(Config { file })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(conf.file)?;
    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}
