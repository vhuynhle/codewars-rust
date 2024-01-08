pub fn max_diff(arr: &[i32]) -> usize {
    (1..arr.len())
        .rev()
        .find(|&diff| (0..(arr.len() - diff)).any(|i| arr[i] <= arr[i + diff]))
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use crate::max_diff;

    #[test]
    fn test_max_diff() {
        assert_eq!(max_diff(&[0, 1, 2, 3]), 3);
        assert_eq!(max_diff(&[3, 1, 2, 3]), 3);
        assert_eq!(max_diff(&[4, 1, 2, 3]), 2);
        assert_eq!(max_diff(&[5, 1, 4, 0]), 1);
        assert_eq!(max_diff(&[3, 2, 1]), 0);
    }
}
