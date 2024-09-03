use buffer::Buffer;
use block::{BlockMD5, BlockParseError};
use raw::RawData;

pub mod raw;
pub mod block;
pub mod buffer;


struct MD5 {
    blocks: Vec<BlockMD5>,
    buffer: Buffer,
    calculated: bool
}

impl MD5 {
    pub fn new(raw_data: RawData) -> Result<Self, BlockParseError> {
        let blocks_result: Result<Vec<BlockMD5>, BlockParseError> = raw_data.try_into();
        match blocks_result {
            Ok(blocks) => Ok(
                MD5 { 
                    blocks,
                    buffer: Buffer::new(),
                    calculated: false
                }),
            Err(err) => Err(err),
        }
    }

    fn calculate_digest(&mut self){
        if self.calculated {
            return
        }
        for block in &self.blocks{
            self.buffer.process_block(block);
        }
    }

    pub fn get_digest(& mut self) -> String{
        if !self.calculated {
            self.calculate_digest();
        }
        self.buffer.get_digest()
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use raw::RawData;

    #[test]
    fn test_md5_example1() {
        let raw_data = RawData::from_str(String::from("Este texto tiene 440 bits, 55 bytes, y un relleno 0x 80").as_str()).expect("Conversion failed");
        let mut md5 = MD5::new(raw_data).expect("Failed to create MD5 instance");

        // Calculate digest
        let digest = md5.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "5f5fe9abdaaad9e5da6f9a661fceae81");
    }

    #[test]
    fn test_md5_example2() {
        let raw_data = RawData::from_str(String::from("Hola Mundo").as_str()).expect("Conversion failed");
        let mut md5 = MD5::new(raw_data).expect("Failed to create MD5 instance");

        // Calculate digest
        let digest = md5.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "d501194c987486789bb01b50dc1a0adb");
    }


    #[test]
    fn test_md5_example3() {
        let raw_data = RawData::from_str(String::from("Generando un MD5 de un texto").as_str()).expect("Conversion failed");
        let mut md5 = MD5::new(raw_data).expect("Failed to create MD5 instance");

        // Calculate digest
        let digest = md5.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "5df9f63916ebf8528697b629022993e8");
    }

    #[test]
    fn test_md5_example4() {
        let raw_data = RawData::from_str(String::from("Generando un MDS de un texto").as_str()).expect("Conversion failed");
        let mut md5 = MD5::new(raw_data).expect("Failed to create MD5 instance");

        // Calculate digest
        let digest = md5.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "e14a3ff5b5e67ede599cac94358e1028");
    }
}