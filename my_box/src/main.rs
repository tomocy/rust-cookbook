fn main() {
    let x = 100;
    let y = MyBox::new(x);

    println!(
        "MyBox<int32> dereferences itself to &int32 and the compiler dereferences it with *: {}",
        *y
    );

    let other = MyBox::new(Box::new(Box::new(MyBox::new(String::from("dereference")))));
    println!("Hello, {}", echo(&other));
}

fn echo(s: &str) -> &str {
    s
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_over_again() {
        let x = MyBox::new(Box::new(Box::new(MyBox::new(String::from("hello")))));
        assert_eq!("hello", echo(&x));
    }
}
