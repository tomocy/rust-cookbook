#[allow(dead_code)]
fn factorial(n: usize) -> u32 {
    if n == 0 {
        1
    } else {
        n as u32 * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(factorial(10), 3628800);
    }
}
