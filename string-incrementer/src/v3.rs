pub fn increment_string(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let nine_count = chars
        .iter()
        .rev()
        .position(|ch| *ch != '9')
        .unwrap_or(chars.len());
    let prefix = &chars[..chars.len() - nine_count];

    let increased_prefix = match prefix.iter().last() {
        Some(digit) if digit.is_ascii_digit() => {
            let unchanged_prefix = prefix[..prefix.len() - 1].iter().collect::<String>();
            let increased_digit = digit.to_digit(10).unwrap() + 1;
            unchanged_prefix + &increased_digit.to_string()
        }
        _ => prefix.iter().collect::<String>() + "1",
    };

    increased_prefix + &"0".repeat(nine_count)
}

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest(
            "foobar9999999999999999999999999999",
            "foobar10000000000000000000000000000",
        );
        dotest("", "1");
    }
}
