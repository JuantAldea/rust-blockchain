use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
mod tests;
use generic_array::GenericArray;

#[derive(Copy, Clone, Debug)]
pub struct Transaction {
    pub sender: u64,
    pub recipient: u64,
    pub amount: u64,
}

//#[derive(Hash)]
#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
    timestamp: u128,
    pub proof: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: GenericArray::default(),
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transactions,
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
        for transaction in &self.transactions {
            s.input(transaction.sender.to_be_bytes());
            s.input(transaction.recipient.to_be_bytes());
            s.input(transaction.amount.to_be_bytes());
        }
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
        write!(
            f,
            "index: {:}; timestamp: {:}; proof: {:x}; previous_hash: {:x}; current_hash: {:x};",
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
