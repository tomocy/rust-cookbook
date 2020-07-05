extern crate anagram_dictionary;

fn main() {
    if let Err(err) = anagram_dictionary::run(std::env::args()) {
        eprintln!("failed to run anagram dictionary: {}", err);
        std::process::exit(1);
    }
}
