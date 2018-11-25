mod block;
mod utils;

use crate::block::Block;
use chrono::prelude::*;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

fn main() {
    let block = Block {
        timestamp: Utc::now().timestamp(),
        data: "testing".to_owned().into(),
        prev_block_hash: Sha256Hash::default(),
        hash: Sha256Hash::default()
    };
    block.set_hash();
    println!("{:?}", Sha256Hash::default());
    println!("Hello, world!");
}
