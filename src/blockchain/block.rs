use std::time::{SystemTime, UNIX_EPOCH};

use super::transaction::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u128,
    pub previous_hash: String,
    pub timestamp: u128,
    pub nonce: u128,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn new(transactions: Vec<SignedTransaction>) -> Self {
        Block {
            index: 0,
            previous_hash: String::from("0").repeat(64),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            transactions,
            nonce: 0,
        }
    }

    pub fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(self.previous_hash.bytes());
        bytes.extend(&self.timestamp.to_be_bytes());

        for transaction in &self.transactions {
            bytes.extend(transaction.uxto_hash().bytes());
        }

        bytes.extend(&self.nonce.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

use std::fmt;
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index: {:}; timestamp: {:}; hash: {:}...; proof: {:x}; previous_hash: {:}...;",
            self.index,
            self.timestamp,
            &self.hash()[..10],
            self.nonce,
            &self.previous_hash[..10],
        )?;

        writeln!(f)?;
        writeln!(f, "Transactions:")?;
        for i in 0..self.transactions.len() - 1 {
            writeln!(f, "\t{}: {} ", i, self.transactions[i]).unwrap();
        }

        write!(
            f,
            "\t{}: {} ",
            self.transactions.len() - 1,
            self.transactions.last().unwrap()
        )
    }
}
