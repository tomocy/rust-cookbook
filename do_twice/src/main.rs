fn main() {}

fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_twice() {
        assert_eq!(6, do_twice(add_one, 2));
    }
}
