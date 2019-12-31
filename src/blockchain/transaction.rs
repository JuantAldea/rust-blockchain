use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use super::hashable::*;
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
}

impl Hashable for Transaction {
    fn hash(&self) -> String {
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
            "in_id:{};intx:{}...;trans_time:{:x};s:{}...;r:{}...;a:{};",
            self.input_block_id,
            &self.intx[..10],
            self.timestamp,
            Id::new(&self.sender),
            Id::new(&self.recipient),
            self.amount,
        )
    }
}
