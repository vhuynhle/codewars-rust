pub fn to_camel_case(s: &str) -> String {
    s.split(|ch| ch == '-' || ch == '_')
        .enumerate()
        .map(|(index, s)| camel_case_word(s, index == 0))
        .collect()
}

fn camel_case_word(w: &str, first_word: bool) -> String {
    if w.is_empty() {
        return "".to_string();
    }

    let first_char = w.chars().next().unwrap();
    let first_char_str: String = if first_word {
        first_char.to_string()
    } else {
        first_char.to_uppercase().collect()
    };
    let rest = w[1..].to_lowercase();
    first_char_str + (&rest)
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}
