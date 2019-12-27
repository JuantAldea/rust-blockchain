use std::time::{SystemTime, UNIX_EPOCH};
mod tests;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: u128,
    pub recipient: u128,
    pub amount: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u128,
    pub previous_hash: String,
    pub timestamp: u128,
    pub proof: u128,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: String::from("0").repeat(64),
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transactions,
            proof: 0,
        }
    }

    pub fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(self.previous_hash.bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        for transaction in &self.transactions {
            bytes.extend(&transaction.sender.to_be_bytes());
            bytes.extend(&transaction.recipient.to_be_bytes());
            bytes.extend(&transaction.amount.to_be_bytes());
        }
        bytes.extend(&self.proof.to_be_bytes());

        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
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
            self.previous_hash,
            self.hash()
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
