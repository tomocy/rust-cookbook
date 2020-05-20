fn main() {
    print_largest(&vec![34, 50, 25, 100, 65]);
    print_largest(&vec!['y', 'm', 'a', 'q']);
}

fn print_largest<T: PartialOrd + std::fmt::Debug>(vals: &[T]) {
    let largest = largest(&vals);
    println!("Values: {:?}", vals);
    println!("    The largest is: {:?}", largest);
}

fn largest<T: PartialOrd>(vals: &[T]) -> &T {
    let mut largest = &vals[0];
    for (i, val) in vals.iter().enumerate() {
        if val > largest {
            largest = &vals[i];
        }
    }

    largest
}
