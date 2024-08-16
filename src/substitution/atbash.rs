pub fn atbash(text: &str) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let top = if c.is_ascii_lowercase() { b'z' } else { b'Z' };
            (top - (c as u8 - base)) as char
        } else {
            c
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atbash_with_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(atbash(input), expected);
    }

    #[test]
    fn test_atbash_with_lowercase() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected = "zyxwvutsrqponmlkjihgfedcba";
        assert_eq!(atbash(input), expected);
    }

    #[test]
    fn test_atbash_with_uppercase() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "ZYXWVUTSRQPONMLKJIHGFEDCBA";
        assert_eq!(atbash(input), expected);
    }

    #[test]
    fn test_atbash_with_mixed_case() {
        let input = "HelloWorld";
        let expected = "SvoolDliow";
        assert_eq!(atbash(input), expected);
    }

    #[test]
    fn test_atbash_with_non_alphabetic_characters() {
        let input = "Hello, World! 123";
        let expected = "Svool, Dliow! 123";
        assert_eq!(atbash(input), expected);
    }

    #[test]
    fn test_atbash_with_numbers_only() {
        let input = "1234567890";
        let expected = "1234567890";
        assert_eq!(atbash(input), expected);
    }
}