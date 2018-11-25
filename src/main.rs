mod block;
mod blockchain;
mod utils;

use crate::block::{Block, Sha256Hash};

fn main() {
    let block = Block::new("Testing", Sha256Hash::default());
    println!("{:?}", block);
}
