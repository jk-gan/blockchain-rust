use crate::utils;
use ring::digest;

pub type Sha256Hash = [u8; 32];

pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    // 256-bit (32 bytes) sha256 algorithm
    pub prev_block_hash: Sha256Hash,
    pub hash: Sha256Hash,
}

impl Block {
    pub fn set_hash(&self) {
        let result = digest::digest(&digest::SHA256, &self.headers_in_bytes());
        println!("{:?}", self.headers_in_bytes());
        println!("{:?}", result.as_ref());
    }

    fn headers_in_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(&utils::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend(&self.data);
        vec.extend(&self.prev_block_hash);
        vec
    }
}
