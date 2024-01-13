fn rot13_char(c: char) -> char {
    if c.is_ascii_uppercase() {
        ((c as u8 - b'A' + 13) % 26 + b'A') as char
    } else if c.is_ascii_lowercase() {
        ((c as u8 - b'a' + 13) % 26 + b'a') as char
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
