use super::transaction::*;
use super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: String,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, signature: String) -> Self {
        Self {
            transaction,
            signature,
        }
    }
}

impl Hashable for SignedTransaction {
    fn hash(&self) -> String {
        let mut bytes = vec![];
        bytes.extend(self.transaction.hash().bytes());
        bytes.extend(self.signature.bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

impl fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}tx_hash:{}...;sign:{}...;txout:{}...;",
            self.transaction,
            &self.transaction.hash()[..10],
            &self.signature[..10],
            &self.hash()[..10]
        )
    }
}
