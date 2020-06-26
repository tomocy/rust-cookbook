#[derive(Debug, PartialEq)]
struct Celsius(f64);

impl From<Kelvin> for Celsius {
    fn from(item: Kelvin) -> Self {
        Self(item.0 - 273.15)
    }
}

#[derive(Debug, PartialEq)]
struct Kelvin(f64);

impl From<Celsius> for Kelvin {
    fn from(item: Celsius) -> Self {
        Self(item.0 + 273.15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_celsius_to_kelvin() {
        assert_eq!(Celsius(31.5), Kelvin(304.65).into());
    }

    #[test]
    fn from_kelvin_to_celsius() {
        assert_eq!(Kelvin(297.4), Celsius(24.25).into());
    }
}
