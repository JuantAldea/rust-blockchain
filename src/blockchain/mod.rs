use sha2::{Digest, Sha256};

use crate::block::*;

mod tests;

#[derive(PartialEq, Debug)]
pub enum BlockChainError {
    BlockChainOk,
    HashMismatch,
    ProofOfWorkError,
    IndexMismatch,
}

#[derive(Debug, Default)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            chain: vec![Block::new(0)],
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

    pub fn calculate_proof(
        block: &Block,
        proof: u64,
    ) -> sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize> {
        let mut s = Sha256::new();
        s.input(block.hash());
        s.input(block.proof.to_be_bytes());
        s.input(proof.to_be_bytes());
        s.result()
    }

    pub fn check_proof(block: &Block, proof: u64) -> BlockChainError {
        let proof_of_work = Self::calculate_proof(block, proof);
        let string_hash = format!("{:x}", proof_of_work);
        if &string_hash[string_hash.len() - 4..] == "0000" {
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

        if BlockChain::check_proof(&self.chain[index - 1], self.chain[index].proof)
            != BlockChainError::BlockChainOk
        {
            return BlockChainError::ProofOfWorkError;
        }

        if self.chain[index - 1].index != self.chain[index].index - 1 {
            return BlockChainError::IndexMismatch;
        }

        BlockChainError::BlockChainOk
    }

    pub fn add_block(&mut self, mut block: Block) {
        let last_block = self.chain.last().unwrap();
        block.previous_hash = self.get_last_hash();
        block.index = self.get_last_index() + 1;
        println!("Mining for block {:}", block);
        loop {
            if BlockChain::check_proof(last_block, block.proof) == BlockChainError::BlockChainOk {
                break;
            }
            block.proof += 1;
        }

        self.chain.push(block);
    }

    pub fn get_last_index(&self) -> u64 {
        self.chain.last().unwrap().index
    }

    pub fn get_last_hash(
        &self,
    ) -> sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize> {
        self.chain.last().unwrap().hash()
    }
}

use std::fmt;
impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Genesis block:").unwrap();
        writeln!(f, "{}", self.chain[0]).unwrap();
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            writeln!(
                f,
                "POW[{:}-{:}]: {:x}",
                i - 1,
                i,
                Self::calculate_proof(previous_block, current_block.proof)
            )
            .unwrap();
            writeln!(f, "{}", current_block).unwrap();
        }
        write!(f, "")
    }
}
