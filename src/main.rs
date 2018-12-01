mod block;
mod blockchain;
mod proof_of_work;
mod utils;

use crate::blockchain::Blockchain;
use ring::digest;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Send 1 BTC to A");
    blockchain.add_block("Send 2 more BTC to A");

    for block in blockchain.blocks {
        println!(
            "Prev Hash: {:?}",
            digest::digest(&digest::SHA256, &block.prev_block_hash)
        );
        println!("Block Data: {:?}", String::from_utf8(block.data).unwrap());
        println!("Hash: {:?}", digest::digest(&digest::SHA256, &block.hash));
    }
}
