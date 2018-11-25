mod block;
mod utils;

use crate::block::Block;

fn main() {
    let block = Block::new();
    println!("{:?}", block);
}
