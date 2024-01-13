pub fn increment_string(s: &str) -> String {
    match s.chars().last() {
        Some('9') => format!("{}0", increment_string(&s[..s.len() - 1])),
        Some(digit) if digit.is_ascii_digit() => {
            format!("{}{}", &s[..s.len() - 1], digit.to_digit(10).unwrap() + 1)
        }
        _ => format!("{}1", s),
    }
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
