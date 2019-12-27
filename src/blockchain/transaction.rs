use serde::{Deserialize, Serialize};
use std::fmt;

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
