fn main() {
    let a = String::from("abcd");
    let b = "xyz";
    let longest = longest(&a, &b);

    println!("The two strings: {} and {}", a, b);
    println!("    The longest is: {}", longest);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
