pub fn next_bigger_number(n: u64) -> Option<u64> {
    let mut digits: Vec<u64> = n
        .to_string()
        .as_bytes()
        .iter()
        .rev()
        .map(|&b| (b - b'0') as u64)
        .collect();

    if rearrange(&mut digits) {
        Some(digits_to_num(&digits))
    } else {
        None
    }
}

fn digits_to_num(digits: &[u64]) -> u64 {
    digits.iter().rev().fold(0, |acc, &d| (acc * 10) + d)
}

fn lower_bound(arr: &[u64], threshold: u64) -> Option<usize> {
    let mut lower_bound: Option<u64> = None;
    let mut lower_bound_index: Option<usize> = None;
    arr.iter().enumerate().for_each(|(index, &value)| {
        if value > threshold && (lower_bound.is_none() || lower_bound.unwrap() > value) {
            lower_bound = Some(value);
            lower_bound_index = Some(index);
        }
    });

    lower_bound_index
}

fn rearrange(digits: &mut [u64]) -> bool {
    for i in 1..digits.len() {
        if let Some(j) = lower_bound(&digits[..i], digits[i]) {
            // Swap a digit with the least bigger digit appearing before it
            digits.swap(i, j);

            // Sort the digits [0..i) to get the minimal number
            digits[..i].sort_by(|a, b| b.cmp(a));
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::next_bigger_number;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected result (right).";

    #[test]
    fn sample_tests() {
        assert_eq!(
            next_bigger_number(1234567890),
            Some(1234567908),
            "{ERR_MSG}"
        );
        assert_eq!(
            next_bigger_number(59884848459853),
            Some(59884848483559),
            "{ERR_MSG}"
        );
        assert_eq!(
            next_bigger_number(5433900109457683864),
            Some(5433900109457684368),
            "{ERR_MSG}"
        );
        assert_eq!(next_bigger_number(9), None, "{ERR_MSG}");
        assert_eq!(next_bigger_number(12), Some(21), "{ERR_MSG}");
        assert_eq!(next_bigger_number(513), Some(531), "{ERR_MSG}");
        assert_eq!(next_bigger_number(2017), Some(2071), "{ERR_MSG}");
        assert_eq!(next_bigger_number(414), Some(441), "{ERR_MSG}");
        assert_eq!(next_bigger_number(144), Some(414), "{ERR_MSG}");
    }
}
