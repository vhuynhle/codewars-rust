pub mod solution {
    use std::fmt::Display;

    struct Range {
        start: i32,
        end: i32,
    }

    impl Display for Range {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.end - self.start {
                0 => write!(f, "{}", self.start),
                1 => write!(f, "{},{}", self.start, self.end),
                _ => write!(f, "{}-{}", self.start, self.end),
            }
        }
    }

    pub fn range_extraction(a: &[i32]) -> String {
        if a.is_empty() {
            return "".to_string();
        }
        let mut res = String::new();
        let mut current_range = Range {
            start: a[0],
            end: a[0],
        };

        for &i in &a[1..] {
            if i == current_range.end + 1 {
                current_range.end = i;
            } else {
                res.push_str(&current_range.to_string());
                res.push(',');
                current_range = Range { start: i, end: i };
            }
        }
        res.push_str(&current_range.to_string());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solution::range_extraction(&[]), "");
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
