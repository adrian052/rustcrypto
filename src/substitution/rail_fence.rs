pub fn encrypt(text: &str, rails: usize) -> String {
    if rails == 1 {
        return String::from(text)
    }
    let mut curr_depth = 0;
    let mut down = false;
    let mut matrix = vec![vec!['|';text.len()];rails];
    for (i,c) in text.char_indices(){
        
        matrix[curr_depth][i] = c;

        if curr_depth == 0 || curr_depth == (rails-1) {
            down = !down;
        }
        if down {
            curr_depth+=1;
        }else{
            curr_depth-=1;
        }
    }
    
    let mut result = String::new();
    for row in matrix {
        for &ch in row.iter() {
            if ch != '|' {
                result.push(ch);
            }
        }
    }

    result
}

pub fn decrypt(text: &str, rails: usize) -> String {
    if rails == 1 {
        return String::from(text)
    }

    let len = text.len();
    let mut result = vec![' '; len];


    let cycle_len = 2 * (rails - 1);
    let chars: Vec<char> = text.chars().collect();
    let mut pos = 0;

    for r in 0..rails {
        let mut i = r;
        while i < len {
            result[i] = chars[pos];
            pos += 1;
            let step = if r == 0 || r == rails - 1 {
                cycle_len
            } else {
                cycle_len - 2 * r
            };

            i += step;

            if r != 0 && r != rails - 1 && i < len {
                result[i] = chars[pos];
                pos += 1;
                i += 2 * r;
            }
        }
    }

    result.iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_with_single_row() {
        let input = "HELLOWORLD".to_string();
        let key = 1;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(encrypt(&input, key), expected);
    }

    #[test]
    fn test_encrypt_with_two_rows() {
        let input = "HELLOWORLD".to_string();
        let key = 2;
        let expected = "HLOOLELWRD".to_string();
        assert_eq!(encrypt(&input, key), expected);
    }

    #[test]
    fn test_encrypt_with_three_rows() {
        let input = "HELLOWORLD".to_string();
        let key = 3;
        let expected = "HOLELWRDLO".to_string();
        assert_eq!(encrypt(&input, key), expected);
    }

    #[test]
    fn test_encrypt_with_special_characters() {
        let input = "HELLO, WORLD!".to_string();
        let key = 3;
        let expected = "HOO!EL,WRDL L".to_string();
        assert_eq!(encrypt(&input, key), expected);
    }

    #[test]
    fn test_encrypt_with_large_key() {
        let input = "HELLOWORLD".to_string();
        let key = 10;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(encrypt(&input, key), expected);
    }

    #[test]
    fn test_decrypt_with_single_row() {
        let input = "HELLOWORLD".to_string();
        let key = 1;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(decrypt(&input, key), expected);
    }

    #[test]
    fn test_decrypt_with_two_rows() {
        let input = "HLOOLELWRD".to_string();
        let key = 2;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(decrypt(&input, key), expected);
    }

    #[test]
    fn test_decrypt_with_three_rows() {
        let input = "HOLELWRDLO".to_string();
        let key = 3;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(decrypt(&input, key), expected);
    }

    #[test]
    fn test_decrypt_with_special_characters() {
        let input = "HOO!EL,WRDL L".to_string();
        let key = 3;
        let expected = "HELLO, WORLD!".to_string();
        assert_eq!(decrypt(&input, key), expected);
    }

    #[test]
    fn test_decrypt_with_large_key() {
        let input = "HELLOWORLD".to_string();
        let key = 10;
        let expected = "HELLOWORLD".to_string();
        assert_eq!(decrypt(&input, key), expected);
    }
}
