use block::{Block512, BlockParseError};
use buffer::Buffer;
use raw::RawData;

pub mod buffer;
pub mod block;
pub mod raw;
struct SHA256 {
    blocks: Vec<Block512>,
    buffer: Buffer,
    calculated: bool
}

impl SHA256 {
    pub fn new(raw_data: RawData) -> Result<Self, BlockParseError> {
        let blocks_result: Result<Vec<Block512>, BlockParseError> = raw_data.try_into();
        match blocks_result {
            Ok(blocks) => Ok(
                SHA256 { 
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

    #[test]
    fn test_sha256_example1() {
        let raw_data = RawData::from_str(String::from("hello world").as_str()).expect("Conversion failed");
        let mut sha256 = SHA256::new(raw_data).expect("Failed to create SHA256 instance");
        // Calculate digest
        let digest = sha256.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_sha256_example2() {
        let raw_data = RawData::from_str(String::from("A simple message").as_str()).expect("Conversion failed");
        let mut sha256 = SHA256::new(raw_data).expect("Failed to create SHA256 instance");
        // Calculate digest
        let digest = sha256.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "9a741f76c050657815eadefc360e8cd191edd49f66ecbdedd048a364d8ef5e86");
    }

    #[test]
    fn test_sha256_example3() {
        let raw_data = RawData::from_str(String::from("").as_str()).expect("Conversion failed");
        let mut sha256 = SHA256::new(raw_data).expect("Failed to create SHA256 instance");
        // Calculate digest
        let digest = sha256.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_sha256_example4() {
        let raw_data = RawData::from_str(String::from("Hundreds of companies around the world are using Rust in production today for fast, low-resource, cross-platform solutions. Software you know and love, like Firefox, Dropbox, and Cloudflare, uses Rust. From startups to large corporations, from embedded devices to scalable web services, Rust is a great fit.").as_str()).expect("Conversion failed");
        let mut sha256 = SHA256::new(raw_data).expect("Failed to create SHA256 instance");
        // Calculate digest
        let digest = sha256.get_digest();
        // Compare the result with the expected digest
        assert_eq!(digest, "dc4340b9b80725c74dbac509869131f2b17b4f5735ac03eef286f5ba1e652dfe");
    }

}