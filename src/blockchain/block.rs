use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::signedtransaction::*;
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u128,
    pub previous_block: String,
    pub timestamp: u128,
    pub nonce: u128,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn new(transactions: Vec<SignedTransaction>) -> Self {
        Self {
            index: 0,
            previous_block: String::from("0").repeat(64),
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
        bytes.extend(self.previous_block.bytes());
        bytes.extend(&self.timestamp.to_be_bytes());

        self.transactions
            .iter()
            .for_each(|tx| bytes.extend(tx.hash().bytes()));

        bytes.extend(&self.nonce.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }

    pub fn find_tx<P, T>(&self, value: &T, predicate: P) -> Option<&SignedTransaction>
    where
        P: Fn(&T, &SignedTransaction) -> bool,
    {
        for tx in self.transactions.iter() {
            if predicate(value, tx) {
                return Some(tx);
            }
        }
        None
    }
}

use std::fmt;
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index:{};timestamp:{};hash:{}...;nonce:{:x};previous_block:{:}...;",
            self.index,
            self.timestamp,
            &self.hash()[..10],
            self.nonce,
            &self.previous_block[..10],
        )?;

        writeln!(f)?;
        writeln!(f, "Transactions:")?;
        for i in 0..self.transactions.len() - 1 {
            writeln!(f, "\t{}: {} ", i, self.transactions[i]).unwrap();
        }

        writeln!(
            f,
            "\t{}: {} ",
            self.transactions.len() - 1,
            self.transactions.last().unwrap()
        )
    }
}
