use crate::block::Block;

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        unimplemented!();
    }

    fn add_block(&mut self, data: &str) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(data, prev_block.hash);
        self.blocks.push(new_block);
    }
}
