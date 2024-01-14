fn digits(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut res = vec![];
    while n != 0 {
        res.push(n % 10);
        n /= 10;
    }
    res.reverse();

    res
}

fn digit_powers(n: u64) -> u64 {
    digits(n)
        .iter()
        .enumerate()
        .map(|(index, &digit)| digit.pow(u32::try_from(index).unwrap() + 1))
        .sum()
}

pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b).filter(|&n| n == digit_powers(n)).collect()
}

#[cfg(test)]
mod tests {
    use super::sum_dig_pow;

    fn dotest(a: u64, b: u64, expected: &[u64]) {
        let actual = sum_dig_pow(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1, 10, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        dotest(1, 100, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
        dotest(10, 89, &[89]);
        dotest(10, 100, &[89]);
        dotest(90, 100, &[]);
        dotest(89, 135, &[89, 135]);
    }
}
