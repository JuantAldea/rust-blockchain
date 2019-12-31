use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use super::id::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Transaction {
    pub input_block_id: u128,
    pub intx: String,
    pub sender: String,
    pub recipient: String,
    pub amount: u128,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(
        input_block_id: u128,
        intx: &str,
        sender: &str,
        recipient: &str,
        amount: u128,
    ) -> Self {
        Transaction {
            input_block_id,
            intx: intx.to_string(),
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }

    pub fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(&self.input_block_id.to_be_bytes());
        bytes.extend(self.intx.bytes());
        bytes.extend(self.sender.bytes());
        bytes.extend(self.recipient.bytes());
        bytes.extend(&self.amount.to_be_bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "trans_time:{:x};in_uxto:{}...;s:{}...;r:{}...;a:{};in_id:{};",
            self.timestamp,
            &self.intx[..10],
            Id::new(&self.sender),
            Id::new(&self.recipient),
            self.amount,
            self.input_block_id,
        )
    }
}

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

    pub fn uxto_hash(&self) -> String {
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
            "sign_time:{:x};{}tx_hash:{}...;sign:{}...;out_uxto:{}...;",
            self.timestamp,
            self.transaction,
            &self.transaction.hash()[..10],
            &self.signature[..10],
            &self.uxto_hash()[..10]
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
