fn rot13_char(c: char) -> char {
    if ('A'..='Z').contains(&c) {
        ((c as u8 - 'A' as u8 + 13) % 26 + 'A' as u8) as char
    } else if ('a'..='z').contains(&c) {
        ((c as u8 - 'a' as u8 + 13) % 26 + 'a' as u8) as char
    } else {
        c
    }
}

pub fn rot13(message: &str) -> String {
    message.chars().map(rot13_char).collect()
}

#[cfg(test)]
mod tests {
    use crate::rot13;

    #[test]
    fn test_rot13() {
        assert_eq!(rot13("!@#$%^&*()_+~`"), "!@#$%^&*()_+~`");
        assert_eq!(rot13(""), "");
        assert_eq!(
            rot13("abcdefghijklmnopqrstuvwxyz"),
            "nopqrstuvwxyzabcdefghijklm"
        );
        assert_eq!(
            rot13("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "NOPQRSTUVWXYZABCDEFGHIJKLM"
        );
    }
}
