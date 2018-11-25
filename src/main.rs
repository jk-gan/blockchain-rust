mod block;
mod blockchain;
mod utils;

use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Send 1 BTC to A");
    blockchain.add_block("Send 2 more BTC to A");

    for block in blockchain.blocks {
        println!("Prev Hash: {:?}", block.prev_block_hash);
        println!("Block Data: {:?}", block.data);
        println!("Hash: {:?}", block.hash);
    }
}
