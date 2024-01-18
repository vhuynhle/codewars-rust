pub fn power_mod(x: u64, mut y: i64, n: u64) -> u64 {
    let mut x = x % n;
    let mut res = 1;
    while y != 0 {
        if (y & 1) == 1 {
            res = (res * x) % n;
        }
        x = (x * x) % n;
        y >>= 1;
    }
    res
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(power_mod(2, 3, 5), 3);
        assert_eq!(power_mod(4, 12, 3), 1);
        assert_eq!(power_mod(11, 10, 300), 1);
        assert_eq!(power_mod(11, 100000, 49), 32);
        assert_eq!(power_mod(5, 100000000, 19), 5);
    }
}
