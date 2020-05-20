use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let filename = "username.txt";
    let username = read_username_from_file(filename).expect(&format!(
        "There was a problem reading username from file '{}'",
        filename
    ));
    println!("Username is: {}", username);
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}
