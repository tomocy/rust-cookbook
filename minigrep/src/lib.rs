use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    File::open(&config.filename)?.read_to_string(&mut contents)?;

    let lines = if config.case_sensitive {
        search_case_sensitive(&contents, &config.query)
    } else {
        search_case_insensitive(&contents, &config.query)
    };

    for line in &lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    filename: String,
    query: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &mut std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(fname) => fname,
            None => return Err("failed to get filename"),
        };
        let query = match args.next() {
            Some(query) => query,
            None => return Err("failed to get query"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            filename: filename,
            query: query,
            case_sensitive: case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_search_result_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(contents, query)
        );
    }

    #[test]
    fn one_search_result_case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(contents, query)
        );
    }
}
