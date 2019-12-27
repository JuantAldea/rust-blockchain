use crate::block::*;
use serde::{Deserialize, Serialize};

mod tests;

#[derive(PartialEq, Debug)]
pub enum BlockChainError {
    BlockChainOk,
    HashMismatch,
    ProofOfWorkError,
    IndexMismatch,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    difficulty: usize,
}

impl BlockChain {
    pub fn new(difficulty: usize, transactions: Vec<Transaction>) -> Self {
        BlockChain {
            chain: vec![Block::new(transactions)],
            difficulty,
        }
    }

    pub fn check_chain(&self) -> BlockChainError {
        for i in 0..self.chain.len() {
            let error = self.check_block(i);
            if error != BlockChainError::BlockChainOk {
                return error;
            }
        }
        BlockChainError::BlockChainOk
    }

    pub fn calculate_proof(block: &Block, proof: u128) -> String {
        let mut bytes = vec![];
        bytes.extend(block.hash().bytes());
        bytes.extend(&block.proof.to_be_bytes());
        bytes.extend(&proof.to_be_bytes());
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }

    pub fn check_proof(&self, block: &Block, proof: u128) -> BlockChainError {
        let proof_of_work = Self::calculate_proof(block, proof);
        if proof_of_work[proof_of_work.len() - self.difficulty..] == "0".repeat(self.difficulty) {
            BlockChainError::BlockChainOk
        } else {
            BlockChainError::ProofOfWorkError
        }
    }

    pub fn check_block(&self, index: usize) -> BlockChainError {
        if index == 0 {
            return BlockChainError::BlockChainOk;
        }

        if self.chain[index].previous_hash != self.chain[index - 1].hash() {
            return BlockChainError::HashMismatch;
        }

        if self.check_proof(&self.chain[index - 1], self.chain[index].proof)
            != BlockChainError::BlockChainOk
        {
            return BlockChainError::ProofOfWorkError;
        }

        if self.chain[index - 1].index != self.chain[index].index - 1 {
            return BlockChainError::IndexMismatch;
        }

        BlockChainError::BlockChainOk
    }

    pub fn add_block(&mut self, mut new_block: Block) {
        let last_block = self.chain.last().unwrap();
        new_block.previous_hash = self.get_last_hash();
        new_block.index = self.get_last_index() + 1;
        println!("Mining for block {:}", new_block);

        loop {
            if self.check_proof(last_block, new_block.proof) == BlockChainError::BlockChainOk {
                break;
            }
            new_block.proof += 1;
        }

        self.chain.push(new_block);
    }

    pub fn get_last_index(&self) -> u128 {
        self.chain.last().unwrap().index
    }

    pub fn get_last_hash(&self) -> String {
        self.chain.last().unwrap().hash()
    }
}

use std::fmt;
impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Genesis block:")?;
        write!(f, "{}", self.chain[0])?;
        for i in 1..self.chain.len() {
            writeln!(f)?;
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            writeln!(
                f,
                "POW[{:}-{:}]: {:}",
                i - 1,
                i,
                Self::calculate_proof(previous_block, current_block.proof)
            )?;
            write!(f, "{:}", current_block)?;
        }
        Ok(())
    }
}
