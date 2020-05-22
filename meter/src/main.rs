use std::ops;

fn main() {}

#[derive(Debug, PartialEq)]
struct Meters(f64);

impl ops::Add<Centimeters> for Meters {
    type Output = Meters;

    fn add(self, other: Centimeters) -> Self::Output {
        Meters((self.0 * 100.0 + other.0) / 100.0)
    }
}

struct Centimeters(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let height = Meters(1.6);
        let shoe_height = Centimeters(3.0);
        assert_eq!(Meters(1.63), height + shoe_height);
    }
}
