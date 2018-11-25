use crate::block::{Block, Sha256Hash};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new("Genesis Block", Sha256Hash::default());
        let mut blocks = Vec::new();
        blocks.push(genesis_block);
        Blockchain {
            blocks
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(data, prev_block.hash);
        self.blocks.push(new_block);
    }
}
