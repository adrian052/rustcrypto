use std::collections::HashSet;

pub fn encrypt(text: &str, keyword: &str) -> String {
    let new_alphabet = get_alphabet(keyword);
    text.chars().map(|c|{
        if c.is_alphabetic(){
            if c.is_uppercase(){
                let char_number:usize = (c as u8 - b'A').into();
                new_alphabet.chars().nth(char_number).unwrap()
            }else {
                let char_number:usize = (c.to_uppercase().next().unwrap() as u8 - b'A').into();
                new_alphabet.chars().nth(char_number).unwrap().to_lowercase().next().unwrap()
            }
        }else{
            c
        }
    }).collect()
}



//Private functions
fn get_alphabet(keyword: &str) -> String{    
    let mut fmt_keyword = keyword.to_uppercase();
    fmt_keyword = remove_duplicates(fmt_keyword);
    remove_duplicates(format!("{}{}", fmt_keyword,"ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
}   

fn remove_duplicates(mut s: String) -> String {
    let mut seen = HashSet::new();
    s.retain(|c| seen.insert(c));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_with_simple_text() {
        let text = "Hello";
        let keyword = "key";
        let expected = "Fbjjn";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_mixed_case_text() {
        let text = "HeLLo";
        let keyword = "Key";
        let expected = "FbJJn";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_empty_text() {
        let text = "";
        let keyword = "Key";
        let expected = "";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_empty_keyword() {
        let text = "Hello";
        let keyword = "";
        let expected = "Hello";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_long_keyword() {
        let text = "Short";
        let keyword = "ThisIsAVeryLongKeyword";
        let expected = "Frwcj";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_special_characters() {
        let text = "Hello!";
        let keyword = "Key";
        let expected = "Fbjjn!";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_encrypt_with_numbers_and_symbols() {
        let text = "Hello123!";
        let keyword = "Key";
        let expected = "Fbjjn123!";
        assert_eq!(encrypt(text, keyword), expected);
    }

    #[test]
    fn test_random1() {
        let text = "Zombie Here";
        let keyword = "secret";
        let expected = "Zljeft Dtot";
        assert_eq!(encrypt(text, keyword), expected);
    }
}
