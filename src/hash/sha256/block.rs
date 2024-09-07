use crate::hash::sha256::RawData;

#[derive(Debug)]
pub struct Block512{
    data:[u32; 16]
}

#[derive(Debug)]
pub enum BlockParseError {
    InvalidArrayLength(usize),
    InvalidChunkLength(usize)
}

impl TryFrom<RawData> for Vec<Block512> {
    type Error = BlockParseError;

    fn try_from(value: RawData) -> Result<Self, Self::Error> {
        if value.len()%64 != 0 {
            return Err(BlockParseError::InvalidArrayLength(value.len()))
        }

        let mut blocks = vec![];

        for chunk in value.get_chunks(){
            blocks.push(Block512::new_block(chunk)?);
        }
        Ok(blocks)
    }
}

impl  Block512{
    fn new_block(chunk: &[u8]) -> Result<Block512, BlockParseError> {
        if chunk.len() != 64 {
            return Err(BlockParseError::InvalidChunkLength(chunk.len()));
        }

        let mut u32_values = [0u32; 16];
        for (i, u32_value) in u32_values.iter_mut().enumerate() {
            *u32_value = Block512::combine_u8_to_u32(&chunk[i * 4..i * 4 + 4]);
        }

        Ok(Block512 {
            data: u32_values,
        })
    }

    fn combine_u8_to_u32(bytes: &[u8]) -> u32 {
        (bytes[0] as u32) << 24 |
        (bytes[1] as u32) << 16 |
        (bytes[2] as u32) << 8  |
        (bytes[3] as u32)
    }

    pub fn get_word(&self, idx: usize) -> u32{
        self.data[idx]
    }

    pub fn display_hex_words(&self) {
        for word in &self.data {
            println!("{:08x} ", word);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_try_from_succeds(){
        let raw = RawData::from_str("abc").unwrap();
        let blocks_result: Result<Vec<Block512>, BlockParseError> = raw.try_into();
        let blocks = blocks_result.unwrap();
        let block = &blocks[0];
        assert_eq!(block.data,[2153996897, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0])
    }
}