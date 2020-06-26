fn main() {
    fizzbuzz(20);
}

fn fizzbuzz(n: usize) {
    for i in 0..n {
        if i % 15 == 0 {
            println!("fizzbuzz")
        } else if i % 3 == 0 {
            println!("fizz")
        } else if i % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", i)
        }
    }
}
