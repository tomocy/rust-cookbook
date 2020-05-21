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
    let mut matched = Vec::new();
    for line in contents.lines() {
        if !line.contains(query) {
            continue;
        }

        matched.push(line);
    }

    matched
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matched = Vec::new();
    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) {
            continue;
        }

        matched.push(line);
    }

    matched
}

pub struct Config {
    filename: String,
    query: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }

        Ok(Config {
            filename: args[1].clone(),
            query: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
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

    #[test]
    fn new_config() {
        let filename = String::from("filename");
        let query = String::from("query");
        let args: Vec<String> = vec![String::from("minigrep"), filename.clone(), query.clone()];
        let config = Config::new(&args).unwrap();

        assert_eq!(filename, config.filename);
        assert_eq!(query, config.query);
    }
}
