use std::{slice::Chunks, str::FromStr};

pub struct RawData{
    data: Vec<u8>
}

impl FromStr for RawData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = utf8_to_vec_u8(s);
        let original_length_bits = (bytes.len() as u64) * 8;
        bytes.push(0x80);
        while (bytes.len() * 8) % 512 != 448 {
            bytes.push(0x00);
        }
        bytes.extend_from_slice(&original_length_bits.to_be_bytes());
        let raw_data = RawData { data: bytes };
        Ok(raw_data)
    }
}

fn utf8_to_vec_u8(s: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for ch in s.chars() {
        let mut buf = [0; 4];
        let encoded = ch.encode_utf8(&mut buf);
        bytes.extend_from_slice(encoded.as_bytes());
    }
    bytes
}

impl RawData {
    
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_chunks(&self) -> Chunks<'_, u8>{
        self.data.chunks(64)
    }

    pub fn get_hex(&self) -> String {
        self.data.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_with_padding() {
        let input = "abc";
        let expected_hex = [
            0x61, 0x62, 0x63, 0x80, 0x00, 0x00, 0x00, 0x00,                   
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 
        ];

        let raw_data = RawData::from_str(input).expect("Conversion failed");
        assert_eq!(raw_data.data, expected_hex);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected_hex = [
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  
        ];
        let raw_data = RawData::from_str(input).expect("Conversion failed");
        assert_eq!(raw_data.data, expected_hex);
    }

    #[test]
    fn test_512_bits_input() {
        let input = "a".repeat(55);  // 55 bytes * 8 = 440 bits
        let mut expected_hex: Vec<u8> = input.bytes().collect();
        expected_hex.push(0x80);
        expected_hex.extend(vec![184,1,0,0,0,0,0,0]);
        let raw_data = RawData::from_str(&input).expect("Conversion failed");
        assert_eq!(raw_data.data, expected_hex);
    }


    
}
