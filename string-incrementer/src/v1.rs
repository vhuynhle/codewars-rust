use std::{fmt::Display, str::FromStr};

pub fn increment_string(s: &str) -> String {
    let number_length = s
        .chars()
        .rev()
        .position(|c| !c.is_ascii_digit())
        .unwrap_or(s.len());

    let split_index = s.len() - number_length;
    let prefix = s[..split_index].to_string();

    let mut num = BigNumber::from_str(&s[split_index..]).unwrap();
    num.inc();
    let num_str = num.to_string();

    let padding = if num_str.len() < number_length {
        "0".repeat(number_length - num_str.len())
    } else {
        "".to_string()
    };

    prefix + &padding + &num_str
}

struct BigNumber {
    digits: Vec<u8>,
}

#[derive(Debug)]
enum BigNumberParseError {
    InvalidChar,
}

impl BigNumber {
    fn inc(&mut self) {
        let mut carry = 1;
        for c in self.digits.iter_mut() {
            *c += carry;
            if *c >= 10 {
                *c -= 10;
                carry = 1;
            } else {
                carry = 0;
                break;
            }
        }
        if carry > 0 {
            self.digits.push(carry);
        }
    }
}

impl Display for BigNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", *digit)?;
        }
        Ok(())
    }
}

impl FromStr for BigNumber {
    type Err = BigNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let non_zero_pos = s.chars().position(|c| c != '0').unwrap_or(s.len());
        let digits = s[non_zero_pos..]
            .chars()
            .rev()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d.try_into().unwrap())
                    .ok_or(BigNumberParseError::InvalidChar)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { digits })
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
