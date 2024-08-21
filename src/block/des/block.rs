use std::str::FromStr;
use std::convert::TryInto;
use crate::block::des::key::Key;

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

    const P_TABLE: [u8; 32] = [
        16, 7, 20, 21, 
        29, 12, 28, 17, 
        1, 15, 23, 26, 
        5, 18, 31, 10, 
        2, 8, 24, 14, 
        32, 27, 3, 9, 
        19, 13, 30, 6, 
        22, 11, 4, 25,
    ];

    fn mangler_function(&self, key:Key) -> Result<Self,()>{
        let expanded_result = self.expansion_permutation();
        match expanded_result {
            Ok(expanded) => {
                let xor_with_key = expanded.key_xor(&key);
                let sbox_reduction = xor_with_key.box_s();
                Ok(sbox_reduction.transposition())
            },
            Err(_) => todo!(),
        }
    }


    fn transposition(&self) -> Self {
        let data: u64 = self.data;
        let lower_32_bits = data as u32;

        let mut permuted_bits = 0u32;
        for (i, &pos) in Self::P_TABLE.iter().enumerate() {
            let bit = (lower_32_bits >> (32 - pos)) & 1;
            permuted_bits |= bit << (31 - i);
        }
        Self {
            data: permuted_bits as u64,
        }
    }

    fn box_s(&self) -> Self {
        let s_boxes: [[u8; 64]; 8] = [
            [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
             0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8,
             4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0,
             15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
            [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10,
             3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5,
             0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15,
             13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
            [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8,
             13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1,
             13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7,
             1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
            [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15,
             13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9,
             10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4,
             3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
            [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9,
             14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6,
             4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14,
             11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
            [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11,
             10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8,
             9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6,
             4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
            [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1,
             13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6,
             1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2,
             6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
            [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7,
             1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2,
             7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8,
             2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
        ];

        let mut result: u32 = 0;
        let mut temp_data = self.data;

        for i in 0..8 {
            let block_6_bits = (temp_data & 0b111111) as u8;
            temp_data >>= 6;
            let sbox_output = s_boxes[i][block_6_bits as usize];
            result |= (sbox_output as u32) << (28 - i * 4);
        }

        DesBlock {
            data: result as u64,
        }
    }

    fn key_xor(&self, key:&Key) -> Self{
        DesBlock{
            data: self.data ^ key.get_data()
        }
    }

    fn expansion_permutation(&self) -> Result<Self,()>{

        if (self.data >> 32) != 0 {
            return Err(());
        }

        let expansion_table : [u8; 48] = [
                                                    32,  1,  2,  3,  4,  5,
                                                     4,  5,  6,  7,  8,  9,
                                                     8,  9, 10, 11, 12, 13,
                                                    12, 13, 14, 15, 16, 17,
                                                    16, 17, 18, 19, 20, 21,
                                                    20, 21, 22, 23, 24, 25,
                                                    24, 25, 26, 27, 28, 29,
                                                    28, 29, 30, 31, 32,  1,
                                                ];

        let mut permuted_input: u64 = 0;
        for (i, &position) in expansion_table.iter().enumerate() {
            let bit = (self.data >> (32 - position)) & 1;
            permuted_input |= bit << (47 - i);
        }

        Ok(DesBlock{
            data: permuted_input
        })

    }


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

    #[test]
    fn test_expansion(){
        let right_half = DesBlock{data:0x00000000F0F0F0F0};
        let expanded_result = right_half.expansion_permutation();
        println!("{}",right_half.string_bits());
        match expanded_result {
            Ok(expanded) => assert_eq!(expanded.string_bits(),"0000000000000000011110100001011110100001011110100001011110100001"),
            Err(_) => panic!("Error while expanding"),
        }
    }


}