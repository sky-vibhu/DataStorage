use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    index: u64,
    pub data: String,
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, data: String) {
        let index = self.blocks.len() as u64;
        let block = Block { index, data };
        self.blocks.push(block);
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }
}
