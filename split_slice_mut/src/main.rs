use std::slice;

fn main() {}

fn split_mut(vals: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= vals.len());

    let addr = vals.as_mut_ptr();
    let (left_len, right_len) = (mid, vals.len() - mid);

    unsafe {
        let (left_addr, right_addr) = (addr, addr.offset(left_len as isize));
        (
            slice::from_raw_parts_mut(left_addr, left_len),
            slice::from_raw_parts_mut(right_addr, right_len),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_mut() {
        let mut vals = vec![1, 2, 3, 4, 5];
        let (left, right) = split_mut(&mut vals, 2);
        assert_eq!(vec![1, 2], left);
        assert_eq!(vec![3, 4, 5], right);
    }
}
