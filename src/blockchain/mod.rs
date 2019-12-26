use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

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

    pub fn check_proof(block: &Block, proof: u64) -> BlockChainError {
        let mut s = DefaultHasher::new();
        s.write_u64(calculate_hash(block));
        s.write_u64(block.proof);
        s.write_u64(proof);

        let hash = s.finish().to_string();
        if &hash[hash.len() - 4..] == "0000" {
            BlockChainError::BlockChainOk
        } else {
            BlockChainError::ProofOfWorkError
        }
    }

    pub fn check_block(&self, index: usize) -> BlockChainError {
        if index == 0 {
            return BlockChainError::BlockChainOk;
        }

        if calculate_hash(&self.chain[index - 1]) != self.chain[index].previous_hash {
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
        block.previous_hash = calculate_hash(last_block);
        block.index = last_block.index + 1;

        loop {
            if BlockChain::check_proof(last_block, block.proof) == BlockChainError::BlockChainOk {
                break;
            }
            block.proof += 1;
        }

        self.chain.push(block);
    }
}
