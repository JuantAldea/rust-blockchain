use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

mod tests;

//#[derive(Hash)]
#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: u64,
    timestamp: u128,
    pub proof: u64,
    pub transaction: u64,
}

impl Block {
    pub fn new(transaction: u64) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: 0,
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transaction,
            proof: 0,
        }
    }

    //pub fn get_proof(&self) -> u64 { self.proof }
    //pub fn get_index(&self) -> u64 { self.index }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.previous_hash.hash(state);
        self.timestamp.hash(state);
        self.transaction.hash(state);
        self.proof.hash(state);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
