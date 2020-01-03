use super::block::*;
use super::signedtransaction::*;
use super::transaction::*;
use super::*;

use openssl::rsa::{Padding, Rsa};
use std::fmt;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum BlockChainOperationResult {
    BlockChainOk,
    BlockChainUpdated,
    BlockChainKept,
    HashMismatchError,
    ProofOfWorkError,
    IndexMismatchError,
    DoubleSpendingError,
    TxIdNotFound,
    InTxOwnershipError,
    InTxTooSmallForTransaction,
    InTxTooSmallForTransactionSet,
    SignatureError,
    SourceBlockIsNewerError,
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
        for block in self.chain.iter() {
            let error = self.check_block(block);
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

    pub fn check_block(&self, block: &Block) -> BlockChainOperationResult {
        if self.check_proof(block) != BlockChainOperationResult::BlockChainOk {
            return BlockChainOperationResult::ProofOfWorkError;
        }

        let block_txs_state = self.validate_block_transactions(&block);
        if block_txs_state != BlockChainOperationResult::BlockChainOk {
            return block_txs_state;
        }

        // The Genesis block has no parent, so no ascendance check can be perform.
        if block.index == 0 {
            return BlockChainOperationResult::BlockChainOk;
        }

        // TODO block index as usize??
        let previous_block = &self.chain[block.index as usize - 1];

        if block.index != previous_block.index + 1 {
            return BlockChainOperationResult::IndexMismatchError;
        }

        if block.previous_block != previous_block.hash() {
            return BlockChainOperationResult::HashMismatchError;
        }

        BlockChainOperationResult::BlockChainOk
    }

    pub fn consensus(&mut self, another: BlockChain) -> BlockChainOperationResult{
        if self.chain.len() < another.chain.len() && another.check_chain() == BlockChainOperationResult::BlockChainOk {
            self.chain = another.chain;
            return BlockChainOperationResult::BlockChainUpdated;
        }

        BlockChainOperationResult::BlockChainKept
    }

    /*
    pub fn validate_block_tx(
        &self,
        tx_block: &Block,
        tx_index: usize,
    ) -> BlockChainOperationResult {
        let tx = &tx_block.transactions[tx_index];

        if tx.transaction.input_block_id > tx_block.index {
            return BlockChainOperationResult::SourceBlockIsNewerError;
        }

        let was_output_of = |intx: &String, tx: &SignedTransaction| *intx == tx.hash();
        let source_blocks = self.chain.iter().filter(|block| { block.find_tx(&tx.transaction.intx, was_output_of).is_some() });
        if source_blocks.len() != 1 {}

        let source_block = &self.chain[tx.transaction.input_block_id as usize];

        let source_tx = source_block.find_tx(&tx.transaction.intx, &is_input);
        let source_tx = match source_tx {
            Some(stx) => stx,
            _ => return BlockChainOperationResult::TxIdNotFound,
        };

        if tx.transaction.sender != source_tx.transaction.recipient {
            return BlockChainOperationResult::InTxOwnershipError;
        }

        let is_valid_transaction = BlockChain::validate_transaction_signature(tx);
        if is_valid_transaction != BlockChainOperationResult::BlockChainOk {
            return is_valid_transaction;
        }

        BlockChainOperationResult::BlockChainOk
    }
    */

    pub fn find_txid_in_block(
        &self,
        index: u128,
        txid: &str,
    ) -> Option<&SignedTransaction> {
        self.chain[index as usize]
            .transactions
            .iter()
            .find(|source_tx| txid == source_tx.hash())
    }

    pub fn validate_block_transactions(&self, block: &Block) -> BlockChainOperationResult {
        log::debug!("================== Validating block ======================");
        let transactions = &block.transactions;
        let mut input_hash = HashMap::new();
        for (tx_index, signed_tx) in transactions.iter().enumerate() {
            log::debug!("Validating transaction");
            log::debug!("{}", signed_tx);

            let tx = &signed_tx.transaction;
            if tx.intx == String::from("0").repeat(64) {
                //This is a coinbase transaction
                log::debug!("Coinbase Transaction. No input check needed.");
                return BlockChainOperationResult::BlockChainOk;
            }

            log::debug!("Source BLOCK {}", self.chain[tx.input_block_id as usize]);

            let intx = self.find_txid_in_block(tx.input_block_id, &tx.intx);

            if intx.is_none() {
                log::warn!("Input TXID not found in source block: FAIL");
                return BlockChainOperationResult::TxIdNotFound;
            }

            let intx = intx.unwrap();

            let is_valid_transaction = BlockChain::validate_transaction_signature(signed_tx);
            if is_valid_transaction != BlockChainOperationResult::BlockChainOk {
                log::warn!("==================BLOCK IS INVALID======================");
                return is_valid_transaction;
            }

            log::debug!("Signature is valid");
            log::debug!("Validating INPUTS");

            let is_valid_transaction =
                self.validate_transaction_inputs(block, tx_index, &intx.transaction);

            if is_valid_transaction != BlockChainOperationResult::BlockChainOk {
                log::warn!("==================BLOCK IS INVALID======================");
                return is_valid_transaction;
            }

            log::debug!("Transaction INPUTS. OK");

            let funds_available = input_hash
                .entry(&tx.intx)
                .or_insert(intx.transaction.amount);
            if funds_available.checked_sub(tx.amount).is_none() {
                log::warn!("Remaining INTX funds ({}) are smaller that the transaction requested {}. (INTX < Sum(UXTOS))", *funds_available, tx.amount);
                log::warn!("==================BLOCK IS INVALID======================");
                return BlockChainOperationResult::InTxTooSmallForTransactionSet;
            }

            *funds_available -= tx.amount;
            log::debug!("Transaction set INTX remaining funds:\n{:?}", input_hash);
        }

        log::debug!("==================BLOCK IS VALID======================");
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

    pub fn validate_transaction_inputs(
        &self,
        //tx: &Transaction,
        tx_block: &Block,
        tx_index: usize,
        intx: &Transaction,
    ) -> BlockChainOperationResult {
        let tx = &tx_block.transactions[tx_index].transaction;
        log::debug!("######### Validating transaction: #########");
        log::debug!("{}", tx);
        let source_block = &tx.input_block_id;
        let source_txid = &tx.intx;

        if intx.recipient != tx.sender {
            log::warn!("TXOUT does not belong to sender: FAIL");
            return BlockChainOperationResult::InTxOwnershipError;
        }

        log::debug!("TXOUT belongs to SENDER: OK.");

        for block in &self.chain[*source_block as usize..] {
            let already_used = block.transactions.iter().find(|stx| {
                let tx = &stx.transaction;
                &tx.intx == source_txid && tx_block.index != block.index
            });

            if let Some(previous_tx) = already_used {
                log::warn!(
                    "Double-spending detected: TXID was consumed in block {} by TX: {}. FAIL",
                    block.index,
                    previous_tx,
                );
                return BlockChainOperationResult::DoubleSpendingError;
            }
        }

        log::debug!("TXOUT is an UXTO. OK.");

        if tx.amount > intx.amount {
            log::warn!(
                "UXTO is too small ({}) for the amount requested ({}). FAIL",
                intx.amount,
                tx.amount
            );
            return BlockChainOperationResult::InTxTooSmallForTransaction;
        }

        log::debug!("UXTO has enough funds. OK.");

        BlockChainOperationResult::BlockChainOk
    }

    pub fn mine_block(&mut self, mut new_block: Block) -> BlockChainOperationResult {
        new_block.previous_block = match self.get_last_hash() {
            Some(previous_hash) => previous_hash,
            None => "0".repeat(64),
        };

        new_block.index = match self.get_last_index() {
            Some(previous_index) => previous_index + 1,
            None => 0,
        };

        log::debug!(
            "================== Adding block #{} ======================",
            new_block.index
        );

        let is_valid = self.validate_block_transactions(&new_block);

        if is_valid != BlockChainOperationResult::BlockChainOk {
            return is_valid;
        }

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

        // necessary?
        let is_valid = self.check_block(&new_block);

        if is_valid != BlockChainOperationResult::BlockChainOk {
            return is_valid;
        }

        self.chain.push(new_block);

        BlockChainOperationResult::BlockChainOk
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
