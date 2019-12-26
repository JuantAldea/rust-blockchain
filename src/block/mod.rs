use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
mod tests;
use generic_array::GenericArray;

//#[derive(Hash)]
#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
    timestamp: u128,
    pub proof: u64,
    pub transaction: u64,
}

impl Block {
    pub fn new(transaction: u64) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: GenericArray::default(),
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transaction,
            proof: 0,
        }
    }

    pub fn hash(
        &self,
    ) -> sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize> {
        let mut s = Sha256::new();
        s.input(self.index.to_be_bytes());
        s.input(self.previous_hash);
        s.input(self.timestamp.to_be_bytes());
        s.input(self.transaction.to_be_bytes());
        s.input(self.proof.to_be_bytes());
        s.result()
    }

    //pub fn get_proof(&self) -> u64 { self.proof }
    //pub fn get_index(&self) -> u64 { self.index }
}

use std::fmt;
impl fmt::Display for Block {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index: {:}; timestamp: {:}; proof: {:x}; transaction: {:};\nprevious_hash: {:x};\n current_hash: {:x};",
        self.index, self.timestamp, self.proof, self.transaction, self.previous_hash, self.hash())
    }
}
