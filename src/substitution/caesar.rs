pub fn caesar(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let new_char = ((c as u8 - base + shift) % 26 + base) as char;
            new_char
        } else {
            c
        }
    }).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shift() {
        assert_eq!(caesar("abc", 1), "bcd");
    }

    #[test]
    fn test_shift_wrap_around() {
        assert_eq!(caesar("xyz", 3), "abc");
    }

    #[test]
    fn test_uppercase_shift() {
        assert_eq!(caesar("XYZ", 3), "ABC");
    }

    #[test]
    fn test_mixed_case_shift() {
        assert_eq!(caesar("AbC", 2), "CdE");
    }

    #[test]
    fn test_no_shift() {
        assert_eq!(caesar("Rust", 0), "Rust");
    }
    
    #[test]
    fn test_large_positive_shift() {
        assert_eq!(caesar("abc", 29), "def");
    }

    #[test]
    fn test_non_alphabetic_characters() {
        assert_eq!(caesar("abc-123!", 3), "def-123!");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(caesar("", 5), "");
    }
}
