use std::time::{SystemTime, UNIX_EPOCH};
mod tests;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Transaction {
    pub input_block_id: u128,
    pub input_uxto_hash: String,
    pub sender: u128,
    pub recipient: u128,
    pub amount: u128,
    pub transaction_hash: String,
    //pub signature: String,
}

impl Transaction {
    pub fn new(
        input_block_id: u128,
        input_uxto_hash: &String,
        sender: u128,
        recipient: u128,
        amount: u128,
    ) -> Self {
        let mut bytes = vec![];
        bytes.extend(&input_block_id.to_be_bytes());
        bytes.extend(input_uxto_hash.bytes());

        bytes.extend(&sender.to_be_bytes());
        bytes.extend(&recipient.to_be_bytes());
        bytes.extend(&amount.to_be_bytes());

        Transaction {
            input_block_id,
            input_uxto_hash: input_uxto_hash.clone(),
            sender,
            recipient,
            amount,
            transaction_hash: crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes),
        }
    }

    pub fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(&self.input_block_id.to_be_bytes());
        bytes.extend(self.input_uxto_hash.bytes());

        bytes.extend(&self.sender.to_be_bytes());
        bytes.extend(&self.recipient.to_be_bytes());
        bytes.extend(&self.amount.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
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
            bytes.extend(transaction.hash().bytes());
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
            "s: {}; r: {}; a: {}; in_id: {}; in_uxto: {}; out_uxto: {};",
            self.sender,
            self.recipient,
            self.amount,
            self.input_block_id,
            self.input_uxto_hash,
            self.transaction_hash
        )
    }
}
