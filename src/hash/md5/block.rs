use std::{error::Error, string::ParseError};
use crate::block;

use super::raw::RawData;


#[derive(Debug)]
struct BlockMD5{
    data:[u32; 16]
}

#[derive(Debug)]
enum BlockParseError {
    InvalidArrayLength(usize),
    InvalidChunkLength(usize)
}

impl TryFrom<RawData> for Vec<BlockMD5> {
    type Error = BlockParseError;

    fn try_from(value: RawData) -> Result<Self, Self::Error> {
        if value.len()%64 != 0 {
            return Err(BlockParseError::InvalidArrayLength(value.len()))
        }

        let mut blocks = vec![];

        for chunk in value.get_chunks(){
            blocks.push(BlockMD5::new_block(chunk)?);
        }
        Ok(blocks)
    }
}

impl  BlockMD5{
    fn new_block(chunk:&[u8]) -> Result<BlockMD5, BlockParseError> {
        if chunk.len() != 64 {
            return Err(BlockParseError::InvalidChunkLength(chunk.len()))
        }

        let mut u32_values = [0u32; 16];
        for (i, u32_value) in u32_values.iter_mut().enumerate() {
            *u32_value = BlockMD5::combine_u8_to_u32(&chunk[i * 4..i * 4 + 4]);
        }

        Ok(BlockMD5 {
            data: u32_values,
        })
    }

    fn combine_u8_to_u32(bytes: &[u8]) -> u32 {
        (bytes[0] as u32) << 24 |
        (bytes[1] as u32) << 16 |
        (bytes[2] as u32) << 8  |
        (bytes[3] as u32)
    }
}


#[cfg(test)]
mod tests {
    use core::panic;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_try_from_succeds(){
        let raw = RawData::from_str("abc").unwrap();
        let blocks_result: Result<Vec<BlockMD5>, BlockParseError> = raw.try_into();
        let blocks = blocks_result.unwrap();
        let block = &blocks[0];
        assert_eq!(block.data,[1633837952, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 402653184, 0])
    }

}