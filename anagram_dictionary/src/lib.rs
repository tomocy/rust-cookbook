use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run<T: Iterator<Item = String>>(args: T) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    let dic = Dictionary::from_file(config.fname)?;

    match dic.find_anagrams(&config.word) {
        Some(anagrams) => {
            for anagram in anagrams {
                println!("{}", anagram);
            }
        }
        None => println!("no anagrams found for {}", config.word),
    }

    Ok(())
}

struct Config {
    fname: String,
    word: String,
}

impl Config {
    fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Self, Box<dyn error::Error>> {
        args.next();

        let fname = match args.next() {
            Some(fname) => fname,
            None => return Err(From::from("filename is unspecified")),
        };
        let word = match args.next() {
            Some(word) => word,
            None => return Err(From::from("word is not unspecified")),
        };

        Ok(Self { fname, word })
    }
}

#[derive(Debug, PartialEq)]
struct Dictionary(HashMap<String, Vec<String>>);

impl Dictionary {
    fn from_file<T: AsRef<Path>>(fname: T) -> Result<Self, io::Error> {
        let mut dict = Self::new();

        let file = File::open(fname)?;
        let file = io::BufReader::new(file);

        for line in file.lines() {
            let word = line?;
            dict.add_word(word);
        }

        Ok(dict)
    }

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_word(&mut self, word: String) {
        let sorted = sort_chars(&word);
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    fn find_anagrams(&self, word: &str) -> Option<&Vec<String>> {
        let sorted = sort_chars(word);
        self.0.get(&sorted)
    }
}

fn sort_chars(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        assert_eq!(
            true,
            Config::new(
                ["/program", "fname.txt", "word"]
                    .iter()
                    .map(|arg| arg.to_string())
            )
            .is_ok()
        );
    }

    #[test]
    fn config_without_fname() {
        assert_eq!(
            false,
            Config::new(["/program"].iter().map(|arg| arg.to_string())).is_ok()
        );
    }

    #[test]
    fn config_without_word() {
        assert_eq!(
            false,
            Config::new(["/program", "fname.txt"].iter().map(|arg| arg.to_string())).is_ok()
        );
    }

    #[test]
    fn dictionary_add_word() {
        let mut expected = Dictionary::new();
        expected.0.insert(
            "aet".to_string(),
            vec![
                "ate".to_string(),
                "eat".to_string(),
                "eta".to_string(),
                "tea".to_string(),
            ],
        );
        expected
            .0
            .insert("dorw".to_string(), vec!["word".to_string()]);

        let mut actual = Dictionary::new();

        actual.add_word("ate".to_string());
        actual.add_word("eat".to_string());
        actual.add_word("eta".to_string());
        actual.add_word("tea".to_string());
        actual.add_word("word".to_string());

        assert_eq!(expected, actual);
    }

    #[test]
    fn dictionary_find_anagrams() {
        let expected = vec![
            "ate".to_string(),
            "eat".to_string(),
            "eta".to_string(),
            "tea".to_string(),
        ];

        let mut dict = Dictionary::new();
        dict.add_word("ate".to_string());
        dict.add_word("eat".to_string());
        dict.add_word("eta".to_string());
        dict.add_word("tea".to_string());
        dict.add_word("word".to_string());

        let actual = dict.find_anagrams("eat").unwrap();

        assert_eq!(expected, *actual);
    }

    #[test]
    fn dictionary_find_no_anagrams() {
        let expected = None;

        let mut dict = Dictionary::new();
        dict.add_word("ate".to_string());
        dict.add_word("eat".to_string());
        dict.add_word("eta".to_string());
        dict.add_word("tea".to_string());
        dict.add_word("word".to_string());

        let actual = dict.find_anagrams("aaa");

        assert_eq!(expected, actual);
    }
}
