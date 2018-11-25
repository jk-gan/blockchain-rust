use crate::utils;
use ring::digest;
use chrono::prelude::*;

pub type Sha256Hash = [u8; 32];

#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    // 256-bit (32 bytes) sha256 algorithm
    pub prev_block_hash: Sha256Hash,
    pub hash: Sha256Hash,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: Sha256Hash) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data: data.to_owned().into(),
            prev_block_hash: prev_block_hash,
            hash: Sha256Hash::default()
        };
        block.set_hash();
        block
    }

    pub fn set_hash(&mut self) {
        let result = digest::digest(&digest::SHA256, &self.headers_in_bytes());
        let mut a: Sha256Hash = Default::default();
        a.copy_from_slice(&result.as_ref()[..]);
        self.hash = a;
    }

    fn headers_in_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(&utils::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend(&self.data);
        vec.extend(&self.prev_block_hash);
        vec
    }
}
