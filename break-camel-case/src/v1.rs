pub fn solution(s: &str) -> String {
    let mut words: Vec<String> = vec![];
    let mut word = "".to_string();
    for c in s.chars() {
        if c.is_uppercase() && !word.is_empty() {
            words.push(word);
            word = "".to_string();
        }
        word.push(c);
    }
    if !word.is_empty() {
        words.push(word);
    }

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
