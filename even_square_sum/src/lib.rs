#[allow(dead_code)]
fn even_square_sum(n: usize) -> u32 {
    (0..n).filter(|x| x % 2 == 0).map(|x| (x * x) as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(120, even_square_sum(10));
    }
}
