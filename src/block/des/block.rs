use std::str::FromStr;
use std::convert::TryInto;

struct DesBlock {
    data: u64,
}

impl FromStr for DesBlock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(());
        }

        if !s.is_ascii() {
            return Err(());
        }

        let bytes: [u8; 8] = s.as_bytes().try_into().map_err(|_| ())?;
        let data = u64::from_be_bytes(bytes);
        Ok(DesBlock { data })
    }
}

impl ToString for DesBlock {
    fn to_string(&self) -> String {
        let mut chars = Vec::with_capacity(8);
        let input = self.data;
        for i in 0..8 {
            let byte = (input >> (56 - i * 8)) as u8;
            chars.push(byte as char);
        }
        chars.into_iter().collect()
    }
}

impl DesBlock {
    fn get_data(&self) -> u64 {
        self.data
    }

    fn string_bits(&self) -> String {
        let mut output = String::from("");
        for i in 0..64 {
            let curr_bit = if (self.data >> (63 - i)) & 1 == 1 {'1'} else {'0'};
            output.push(curr_bit);
        }
        output
    }

    fn concat_halfs(left: DesBlock, right: DesBlock) -> Result<DesBlock, ()> {
        if (right.data >> 32) != 0 {
            return Err(());
        }

        if (left.data & 0xFFFFFFFF) != 0 {
            return Err(());
        }

        let concat = (right.data << 32) | left.data;
        Ok(DesBlock { data: concat })
    }

    fn init_permutation(&self) -> Self {
        let ip_table: [u8; 64] = [
            58, 50, 42, 34, 26, 18, 10, 2,
            60, 52, 44, 36, 28, 20, 12, 4,
            62, 54, 46, 38, 30, 22, 14, 6,
            64, 56, 48, 40, 32, 24, 16, 8,
            57, 49, 41, 33, 25, 17, 9, 1,
            59, 51, 43, 35, 27, 19, 11, 3,
            61, 53, 45, 37, 29, 21, 13, 5,
            63, 55, 47, 39, 31, 23, 15, 7,
        ];

        let mut permuted_input: u64 = 0;

        for (i, &position) in ip_table.iter().enumerate() {
            let bit = (self.data >> (64 - position)) & 1;
            permuted_input |= bit << (63 - i);
        }

        DesBlock{
            data: permuted_input
        }
    }

    fn inverse_permutation(&self) -> Self {
        let ip_inverse_table: [u8; 64] = [
            40, 8, 48, 16, 56, 24, 64, 32,
            39, 7, 47, 15, 55, 23, 63, 31,
            38, 6, 46, 14, 54, 22, 62, 30,
            37, 5, 45, 13, 53, 21, 61, 29,
            36, 4, 44, 12, 52, 20, 60, 28,
            35, 3, 43, 11, 51, 19, 59, 27,
            34, 2, 42, 10, 50, 18, 58, 26,
            33, 1, 41, 9, 49, 17, 57, 25,
        ];

        let mut permuted_input: u64 = 0;

        for (i, &position) in ip_inverse_table.iter().enumerate() {
            let bit = (self.data >> (64 - position)) & 1;
            permuted_input |= bit << (63 - i);
        }

        DesBlock{
            data: permuted_input
        }
    }

    fn bits_swap32(&self) -> Self {
        let upper = self.data >> 32;
        let lower = self.data & 0xFFFFFFFF;
        let swapped = (lower as u64) << 32 | upper;
        DesBlock {
            data: swapped
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_from_str_wrong_size() {
        let input = "ABCDEFGHS";
        assert!(input.parse::<DesBlock>().is_err()); 
    }

    #[test]
    fn test_block_from_str_wrong_non_ascii() {
        let input = "ABCDEFG👌";
        assert!(input.parse::<DesBlock>().is_err()); 
    }

    #[test]
    fn test_block_from_str_succeds1() {
        let input = "ABCDEFGH";
        assert!(input.parse::<DesBlock>().is_ok()); 
    }

    #[test]
    fn test_block_from_str_succeds2() {
        let input = "abcdefgh";
        assert!(input.parse::<DesBlock>().is_ok()); 
    }

    #[test]
    fn test_block_from_and_to_string_succeds() {
        let input = String::from("abcdefgh");
        let block = input.parse::<DesBlock>().unwrap();
        let output = block.to_string();
        assert_eq!(input,output); 
    }

    #[test]
    fn test_string_bits(){
        let input = String::from("abcdefgh");
        let block = input.parse::<DesBlock>().unwrap();
        assert_eq!(block.string_bits(), "0110000101100010011000110110010001100101011001100110011101101000");
    }

    #[test]
    fn test_swap_bits(){
        let input = String::from("abcdefgh");
        let mut block = input.parse::<DesBlock>().unwrap();
        block = block.bits_swap32();
        assert_eq!(block.string_bits(), "0110010101100110011001110110100001100001011000100110001101100100");
    }


}