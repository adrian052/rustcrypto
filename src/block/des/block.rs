use core::panic;
use std::str::FromStr;
use std::convert::TryInto;
use crate::block::des::key::Key;
use crate::block::des::tables::*;

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


impl DesBlock {
    pub fn encrypt_block(&self, key: &Key) -> Self {
        let mut result = self.init_permutation();
        let subkeys = key.generate_subkeys();
        for subkey in subkeys {
            result = result.round_des(subkey);
        }

        result = result.bits_swap32().inverse_permutation();
        result
    }

    pub fn decrypt_block(&self, key: &Key) -> Self {
        let mut result = self.init_permutation();
        let subkeys = key.generate_subkeys();
        for subkey in subkeys.iter().rev() {
            result = result.round_des(subkey.clone());
        }
        result = result.bits_swap32().inverse_permutation();
        result
    }

    fn round_des(&self, key: Key) -> Self {
    let right = DesBlock { data: self.data & 0xFFFFFFFF };
    let left = DesBlock { data: self.data >> 32 };

    match right.mangler_function(key) {
        Ok(mangled) => {
            let new_right = left.data ^ mangled.data;
            let new_data = (right.data << 32) | new_right;
            let result = DesBlock {
                data: new_data
            };
            result
        }
        Err(_) => panic!("Error in mangler function"),
    }
}


    fn mangler_function(&self, key: Key) -> Result<Self, ()> {
        let result = self.expansion_permutation()?
                                    .key_xor(&key)
                                    .box_s()
                                    .transposition();
        Ok(result)
    }

    fn to_hex_string(&self) -> String {
        let mut result = String::with_capacity(16);
        let input = self.data;

        for i in 0..8 {
            let byte = (input >> (56 - i * 8)) as u8;
            result.push_str(&format!("{:02X}", byte));
        }
        result.insert_str(0, "0x");
        result
    }

    fn transposition(&self) -> Self {
        let data: u64 = self.data;
        let lower_32_bits = data as u32;

        let mut permuted_bits = 0u32;
        for (i, &pos) in P_TABLE.iter().enumerate() {
            let bit = (lower_32_bits >> (32 - pos)) & 1;
            permuted_bits |= bit << (31 - i);
        }
        Self {
            data: permuted_bits as u64,
        }
    }

    pub fn box_s(&self) -> Self {
        let mut output: u32 = 0;
        
        for i in 0..8 {
            let block = (self.data >> (42 - 6 * i)) & 0x3F;
            let row = ((block & 0x20) >> 4) | (block & 0x01);
            let column = (block >> 1) & 0x0F;
            let sbox_value = SBOX[i][row as usize][column as usize] as u32;            
            output |= sbox_value << (28 - 4 * i);
        }
        
        DesBlock{data:output as u64}
    }
    
    

    fn key_xor(&self, key: &Key) -> Self {
        DesBlock {
            data: self.data ^ key.get_data(),
        }
    }

    fn expansion_permutation(&self) -> Result<Self, ()> {
        if (self.data >> 32) != 0 {
            return Err(());
        }

        let mut permuted_input: u64 = 0;
        for (i, &position) in EXPANSION_TABLE.iter().enumerate() {
            let bit = (self.data >> (32 - position)) & 1;
            permuted_input |= bit << (47 - i);
        }

        Ok(DesBlock {
            data: permuted_input,
        })
    }

    fn string_bits(&self) -> String {
        let mut output = String::from("");
        for i in 0..64 {
            let curr_bit = if (self.data >> (63 - i)) & 1 == 1 { '1' } else { '0' };
            output.push(curr_bit);
        }
        output
    }

    fn init_permutation(&self) -> Self {
        let mut permuted_input: u64 = 0;

        for (i, &position) in IP_TABLE.iter().enumerate() {
            let bit = (self.data >> (64 - position)) & 1;
            permuted_input |= bit << (63 - i);
        }
        DesBlock {
            data: permuted_input,
        }
    }

    fn inverse_permutation(&self) -> Self {
        let mut permuted_input: u64 = 0;
        for (i, &position) in IP_INVERSE_TABLE.iter().enumerate() {
            let bit = (self.data >> (64 - position)) & 1;
            permuted_input |= bit << (63 - i);
        }

        DesBlock {
            data: permuted_input,
        }
    }

    fn bits_swap32(&self) -> Self {
        let upper = self.data >> 32;
        let lower = self.data & 0xFFFFFFFF;
        let swapped = (lower as u64) << 32 | upper;
        DesBlock {
            data: swapped,
        }
    }

    fn get_right_bits(&self) -> Self {
        let mask: u64 = 0xFFFFFFFF;
        DesBlock { data: self.data & mask }
    }

    pub fn from_64bits_number(number:u64) -> Self{
        DesBlock{
            data: number
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
        match expanded_result {
            Ok(expanded) => assert_eq!(expanded.string_bits(),"0000000000000000011110100001011110100001011110100001011110100001"),
            Err(_) => self::panic!("Error while expanding"),
        }
    }

    #[test]
    fn test_mangler_function(){
        let key = Key::mock_key();
        let block = String::from("ABCDEFGH").parse::<DesBlock>().unwrap().get_right_bits();
        let result = block.mangler_function(key).unwrap();
        assert_eq!(result.to_hex_string(), "0x000000007BB1FF34");
    }

    #[test]
    fn test_single_round(){
        let block = String::from("abcdefgh").parse::<DesBlock>().unwrap();
        let round1 = block.round_des(Key::mock_key());
        assert_eq!(round1.string_bits(), "0110010101100110011001110110100000011000110101100001100011010111")
    }

    #[test]
    fn test_encryption(){
        let block = DesBlock::from_64bits_number(0x123456ABCD132536);
        let key = Key::from_64bits_number(0xAABB09182736CCDD);
        let encrypted = block.encrypt_block(&key);
        assert_eq!(encrypted.to_hex_string(), "0xC0B7A8D05F3A829C");
        let decrypted = encrypted.decrypt_block(&key);
        assert_eq!(decrypted.to_hex_string(), "0x123456ABCD132536");
    }

}