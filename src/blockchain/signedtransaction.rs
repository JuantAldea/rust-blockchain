use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use super::hashable::*;
use super::transaction::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub timestamp: u128,
    pub signature: String,
}

impl SignedTransaction {
    pub fn new(transaction: &Transaction) -> Self {
        Self {
            transaction: transaction.clone(),
            signature: String::from("0"),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }

    pub fn hash_for_signature(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(self.transaction.hash().bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(self.transaction.hash().bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(self.signature.bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

impl fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sign_time:{:x};{}tx_hash:{}...;sign:{}...;txout:{}...;",
            self.timestamp,
            self.transaction,
            &self.transaction.hash()[..10],
            &self.signature[..10],
            &self.hash()[..10]
        )
    }
}

pub struct TransactionsVec(pub Vec<Transaction>);

impl fmt::Display for TransactionsVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.len() {
            writeln!(f, "{}: {}", i, self.0[i]).unwrap();
        }
        Ok(())
    }
}
