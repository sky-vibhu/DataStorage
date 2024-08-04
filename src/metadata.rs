use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub chunk_hashes: Vec<String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self { chunk_hashes: Vec::new() }
    }

    pub fn add_chunk_hash(&mut self, hash: String) {
        self.chunk_hashes.push(hash);
    }
}



