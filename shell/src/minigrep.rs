use std::fs;
use std::error::Error;
use std::convert::AsRef;

const MIN_ARGS: usize = 3;
const MAX_ARGS: usize = 4;


pub struct Config {
    query: String,
    file: String,
    case_sensitive: bool,
    invert_match: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < MIN_ARGS || args.len() > MAX_ARGS {
            return Err("too few arguments");
       }

        //This is default config
        let query = args[args.len()- 2].clone();
        let file = args[args.len() - 1].clone();
        let mut case_sensitive = false;
        let mut invert_match = false;

        //Options
        match args[1].as_ref() {
            "-i" => case_sensitive = true,
            "-v" => invert_match = true,
            _ => ()
        }
        Ok(Config { query, file, case_sensitive, invert_match})
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(conf.file)?;

    if conf.case_sensitive {
        for line in search_case_insensitive(&conf.query, &content) {
            println!("{}", line);
        }
    } else if conf.invert_match {
        for line in search_invert_match(&conf.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in search(&conf.query, &content) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub fn search_invert_match<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if !line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

/* TODO
 * pub fn search_word_match<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
 * }
 *
 */


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duc";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt:";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:"],
            search_case_insensitive(query, content)
            );
    }

    #[test]
    fn invert_match() {
        let query = "duc";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "Pick three."],
            search_invert_match(query, content)
            );
    }

    #[test]
    fn word_match() {
        let query = "Pick";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Pick"],
            search_word_match(query, content)
            );
    }
}
