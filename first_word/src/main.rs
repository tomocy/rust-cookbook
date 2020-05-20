fn main() {
    let s1 = String::from("Hello, world");
    let s1_first_word = first_word(&s1);
    println!("the first world of '{}' is {}", s1, s1_first_word);

    let s2 = "In the world";
    let s2_first_word = first_word(s2);
    println!("the first world of '{}' is {}", s2, s2_first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, byte) in bytes.iter().enumerate() {
        if *byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
