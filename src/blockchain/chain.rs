use super::block::*;
use super::transaction::*;
use openssl::rsa::{Padding, Rsa};
use std::fmt;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum BlockChainOperationResult {
    BlockChainOk,
    HashMismatchError,
    ProofOfWorkError,
    IndexMismatchError,
    DoubleExpendingError,
    InTxNotFound,
    InTxOwnershipError,
    InTxTooSmallForTransaction,
    InTxTooSmallForTransactionSet,
    SignatureError,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    difficulty: usize,
}

impl BlockChain {
    pub fn new(difficulty: usize) -> Self {
        BlockChain {
            chain: vec![],
            difficulty,
        }
    }

    pub fn check_chain(&self) -> BlockChainOperationResult {
        for i in 0..self.chain.len() {
            let error = self.check_block(i);
            if error != BlockChainOperationResult::BlockChainOk {
                return error;
            }
        }
        BlockChainOperationResult::BlockChainOk
    }

    pub fn check_proof(&self, block: &Block) -> BlockChainOperationResult {
        let proof_of_work = block.hash();
        log::trace!("Checking nonce: {} -> PoW: {}", block.nonce, proof_of_work);
        if proof_of_work[..self.difficulty] != "0".repeat(self.difficulty) {
            return BlockChainOperationResult::ProofOfWorkError;
        }

        BlockChainOperationResult::BlockChainOk
    }

    pub fn check_block(&self, index: usize) -> BlockChainOperationResult {
        if self.check_proof(&self.chain[index]) != BlockChainOperationResult::BlockChainOk {
            return BlockChainOperationResult::ProofOfWorkError;
        }

        // The Genesis block has no parent, so no ascendance check can be perform.
        if index == 0 {
            return BlockChainOperationResult::BlockChainOk;
        }

        if self.chain[index].index != self.chain[index - 1].index + 1 {
            return BlockChainOperationResult::IndexMismatchError;
        }

        if self.chain[index].previous_hash != self.chain[index - 1].hash() {
            return BlockChainOperationResult::HashMismatchError;
        }

        BlockChainOperationResult::BlockChainOk
    }

    pub fn validate_transaction_signature(
        signed_tx: &SignedTransaction,
    ) -> BlockChainOperationResult {
        let sender = &signed_tx.transaction.sender;
        let transaction_hash = signed_tx.transaction.hash();
        let transaction_signature_decoded = bs58::decode(&signed_tx.signature).into_vec().unwrap();

        let decoded_key = bs58::decode(sender).into_vec().unwrap();
        let rsa_public = Rsa::public_key_from_der(&decoded_key).unwrap();
        let mut buf: Vec<u8> = vec![0u8; 2048];
        let _len = rsa_public
            .public_decrypt(&transaction_signature_decoded, &mut buf, Padding::PKCS1)
            .unwrap();
        let decrypted_hash = String::from_utf8(buf).unwrap();

        if transaction_hash == decrypted_hash {
            log::warn!("Invalid signature: FAIL");
            return BlockChainOperationResult::SignatureError;
        }

        BlockChainOperationResult::BlockChainOk
    }

    pub fn validate_block(&self, block: &Block) -> BlockChainOperationResult {
        log::debug!("================== Validating block ======================");
        let mut input_hash = HashMap::new();
        for signed_tx in &block.transactions {
            log::debug!("Validating transaction");
            log::debug!("{}", signed_tx);

            let tx = &signed_tx.transaction;
            log::debug!("Source BLOCK {}", self.chain[tx.input_block_id as usize]);

            let intx = self.chain[tx.input_block_id as usize]
                .transactions
                .iter()
                .find(|source_tx| source_tx.uxto_hash() == tx.intx);

            if intx.is_none() {
                log::warn!("UXTO not found in source block: FAIL");
                return BlockChainOperationResult::InTxNotFound;
            }

            let intx = intx.unwrap();

            let funds_available = input_hash
                .entry(&tx.intx)
                .or_insert(intx.transaction.amount);

            let is_valid_transaction = BlockChain::validate_transaction_signature(signed_tx);
            if is_valid_transaction != BlockChainOperationResult::BlockChainOk {
                log::warn!("==================BLOCK IS INVALID======================");
                return is_valid_transaction;
            }

            log::debug!("Signature is valid");
            log::debug!("Validating INPUTS");

            let is_valid_transaction = self.validate_transaction_inputs(&tx, &intx.transaction);
            if is_valid_transaction != BlockChainOperationResult::BlockChainOk {
                log::warn!("==================BLOCK IS INVALID======================");
                return is_valid_transaction;
            }

            log::debug!("Transaction INPUTS. OK");

            if funds_available.checked_sub(tx.amount).is_none() {
                log::warn!("Remaining InTX funds ({}) are smaller that the transaction requested {}. (InTX < Sum(UXTOS))", *funds_available, tx.amount);
                log::warn!("==================BLOCK IS INVALID======================");
                return BlockChainOperationResult::InTxTooSmallForTransactionSet;
            }

            *funds_available -= tx.amount;
            log::debug!("Transaction set InTX remaining funds:\n{:?}", input_hash);
        }

        log::debug!("==================BLOCK IS VALID======================");
        BlockChainOperationResult::BlockChainOk
    }

    pub fn validate_transaction_inputs(
        &self,
        tx: &Transaction,
        intx: &Transaction,
    ) -> BlockChainOperationResult {
        log::debug!("######### Validating transaction: #########");
        log::debug!("{}", tx);
        let source_block = &tx.input_block_id;
        let source_uxto = &tx.intx;

        if intx.recipient != tx.sender {
            log::warn!("UXTO does not belong to sender: FAIL");
            return BlockChainOperationResult::InTxOwnershipError;
        }

        log::debug!("UXTO belongs to SENDER: OK.");

        for block in &self.chain[*source_block as usize..] {
            for signed_transaction in &block.transactions {
                let transaction = &signed_transaction.transaction;
                if &transaction.intx == source_uxto {
                    log::warn!(
                        "UXTO was consumed in block {}. Double Expending detected: FAIL",
                        block.index
                    );
                    return BlockChainOperationResult::DoubleExpendingError;
                }
            }
        }

        log::debug!("UXTO is available. OK.");

        if tx.amount > intx.amount {
            log::warn!(
                "Input UXTO is too small ({}) for the requested amount ({}). FAIL",
                intx.amount,
                tx.amount
            );
            return BlockChainOperationResult::InTxTooSmallForTransaction;
        }

        log::debug!("UXTO has enough funds. OK.");

        BlockChainOperationResult::BlockChainOk
    }

    pub fn mine_block(&mut self, mut new_block: Block) {
        new_block.previous_hash = match self.get_last_hash() {
            Some(previous_hash) => previous_hash,
            None => "0".repeat(64),
        };

        new_block.index = match self.get_last_index() {
            Some(previous_index) => previous_index + 1,
            None => 0,
        };

        log::debug!("Mining for block #{}:", &new_block.index);
        log::debug!("{}", &new_block);

        loop {
            if self.check_proof(&new_block) == BlockChainOperationResult::BlockChainOk {
                break;
            }
            new_block.nonce += 1;
        }

        log::debug!(
            "Nonce found: {:x} => H[B] = {}",
            new_block.nonce,
            new_block.hash()
        );

        self.chain.push(new_block);
    }

    pub fn get_last_index(&self) -> Option<u128> {
        if self.chain.is_empty() {
            return None;
        }

        Some(self.chain.last().unwrap().index)
    }

    pub fn get_last_hash(&self) -> Option<String> {
        if self.chain.is_empty() {
            return None;
        }

        Some(self.chain.last().unwrap().hash())
    }
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.chain.is_empty() {
            return write!(f, "Empty chain");
        }

        for i in 0..self.chain.len() - 1 {
            let current_block = &self.chain[i];
            writeln!(f, "Block: {}: {}", i, current_block)?;
        }

        write!(
            f,
            "Block: {}: {}",
            self.chain.len() - 1,
            self.chain.last().unwrap()
        )
    }
}
