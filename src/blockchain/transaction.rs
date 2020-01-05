use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use super::id::*;
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct InputData {
    pub block_id: u128,
    pub index: u128,
    pub txid: String,
    pub amount: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct OutputData {
    pub index: u128,
    pub recipient: String,
    pub amount: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Transaction {
    //pub input_block_id: u128,
    //pub intx: String,
    pub inputs: Vec<InputData>,
    pub sender: String,
    //pub recipient: String,
    //pub amount: u128,
    pub outputs: Vec<OutputData>,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(
        //input_block_id: u128,
        //intx: &str,
        inputs: Vec<InputData>,
        sender: &str,
        //recipient: &str,
        //amount: u128,
        outputs: Vec<OutputData>,
    ) -> Self {
        Self {
            //input_block_id,
            //intx: intx.to_string(),
            inputs,
            sender: sender.to_string(),
            //recipient: recipient.to_string(),
            //amount,
            outputs,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }

    pub fn sum_inputs(&self) -> u128 {
        self.inputs.iter().fold(0, |sum, input| sum + input.amount)
    }

    pub fn sum_outputs(&self) -> u128 {
        self.outputs.iter().fold(0, |sum, ouput| sum + ouput.amount)
    }

    pub fn validate(&self) -> BlockChainOperationResult {
        if self.sum_inputs() < self.sum_outputs() {
            return BlockChainOperationResult::TXSumError;
        }

        BlockChainOperationResult::BlockChainOk
    }

    pub fn transaction_fee(&self) -> u128 {
        self.sum_inputs() - self.sum_outputs()
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> String {
        let mut bytes = vec![];
        //bytes.extend(&self.input_block_id.to_be_bytes());
        for input in self.inputs.iter(){
            bytes.extend(&input.block_id.to_be_bytes());
            bytes.extend(&input.index.to_be_bytes());
            bytes.extend(input.txid.bytes());
            //bytes.extend(input.amount.bytes());
        }
        //bytes.extend(&self.inputs.to_be_bytes()),
        //bytes.extend(self.intx.bytes());
        bytes.extend(self.sender.bytes());
        //bytes.extend(self.recipient.bytes());
        //bytes.extend(&self.amount.to_be_bytes());
        for output in self.outputs.iter(){
            bytes.extend(&output.index.to_be_bytes());
            bytes.extend(output.recipient.bytes());
            bytes.extend(&output.amount.to_be_bytes());
        }

        bytes.extend(&self.timestamp.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "trans_time:{:x}; sender:{};", self.timestamp, Id::new(&self.sender));
        writeln!(f, "Inputs");
        for input in self.inputs {
            writeln!(f, "\tBId:{:x}; #:{} intx:{}; amount: {}", input.block_id, input.index, input.txid, input.amount);
        }

        writeln!(f, "Outputs");
        for output in self.outputs {
            writeln!(f, "\t#{}: recipient:{}; amount: {}", output.index, output.recipient, output.amount);
        }

        writeln!(f, "Fee: {}", self.transaction_fee())
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
