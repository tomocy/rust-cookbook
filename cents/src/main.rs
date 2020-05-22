fn main() {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quater => 25,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value() {
        assert_eq!(1, Coin::Penny.value_in_cents());
        assert_eq!(5, Coin::Nickel.value_in_cents());
        assert_eq!(10, Coin::Dime.value_in_cents());
        assert_eq!(25, Coin::Quater.value_in_cents());
    }
}
