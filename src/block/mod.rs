use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
mod tests;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: u128,
    pub recipient: u128,
    pub amount: u128,
}

pub type SHA256Hash = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u128,
    pub previous_hash: SHA256Hash,
    timestamp: u128,
    pub proof: u128,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: [0u8; 32],
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transactions,
            proof: 0,
        }
    }

    pub fn hash(&self) -> SHA256Hash {
        let mut s = Sha256::new();
        s.input(self.index.to_be_bytes());
        s.input(self.previous_hash);
        s.input(self.timestamp.to_be_bytes());
        for transaction in &self.transactions {
            s.input(transaction.sender.to_be_bytes());
            s.input(transaction.recipient.to_be_bytes());
            s.input(transaction.amount.to_be_bytes());
        }
        s.input(self.proof.to_be_bytes());
        s.result().into()
    }

    //pub fn get_proof(&self) -> u64 { self.proof }
    //pub fn get_index(&self) -> u64 { self.index }
}

use std::fmt;
impl fmt::Display for Block {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index: {:}; timestamp: {:}; proof: {:x}; previous_hash: {:}; current_hash: {:};",
            self.index,
            self.timestamp,
            self.proof,
            hex::encode(self.previous_hash),
            hex::encode(self.hash())
        )?;

        writeln!(f)?;
        write!(f, "Transactions:")?;

        self.transactions
            .iter()
            .enumerate()
            .for_each(|(i, transaction)| {
                write!(f, "\n\t{}: {} ", i, transaction).unwrap();
            });

        Ok(())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sender: {}; recipient: {}; amount: {};",
            self.sender, self.recipient, self.amount
        )
    }
}
