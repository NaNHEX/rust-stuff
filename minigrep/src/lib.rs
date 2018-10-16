use std::error::Error;

pub struct Config {
    query: String,
    file: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments");
        }

        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config { query, file })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(conf.file)?;

    for line in search(&conf.query, &content) {
        println!("{}", line);
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
}
