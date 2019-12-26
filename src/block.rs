use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

#[derive(PartialEq, Debug)]
pub enum BlockChainError {
    BlockChainOk,
    HashMismatch,
    ProofOfWorkError,
    IndexMismatch,
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

//#[derive(Hash)]
#[derive(Copy, Clone, Debug)]
pub struct Block {
    index: u64,
    previous_hash: u64,
    timestamp: u128,
    proof: u64,
    transaction: u64,
}

impl Block {
    pub fn new(transaction: u64) -> Self {
        let start = SystemTime::now();
        Block {
            index: 0,
            previous_hash: 0,
            timestamp: start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            transaction,
            proof: 0,
        }
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.previous_hash.hash(state);
        self.timestamp.hash(state);
        self.transaction.hash(state);
        self.proof.hash(state);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn block_hash() {
        let block = Block {
            index: 1,
            previous_hash: 2,
            timestamp: 3,
            transaction: 4,
            proof: 5,
        };
        let hash = calculate_hash(&block);
        assert_eq!(10219695379388903649, hash)
    }

    #[test]
    fn test_chain_break_proof() {
        let mut chain = BlockChain::new();
        let block1 = Block::new(128);

        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.chain[1].proof = 0;

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::ProofOfWorkError);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::ProofOfWorkError)

    }

    #[test]
    fn test_chain_break_index() {
        let mut chain = BlockChain::new();
        let block1 = Block::new(128);

        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.chain[2].index = 1;

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::IndexMismatch);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::IndexMismatch)
    }

    #[test]
    fn test_chain_break_previous_hash() {
        let mut chain = BlockChain::new();
        let block1 = Block::new(128);

        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.chain[1].previous_hash = 0;

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::HashMismatch);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::HashMismatch)
    }
    #[test]
    fn test_chain_break_transaction() {
        let mut chain = BlockChain::new();
        let block1 = Block::new(128);

        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);

        chain.chain[1].transaction = 2;

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::HashMismatch);

        chain.add_block(block1);
        println!("{:?}", chain);
        println!("{:?}", chain.check_chain());

        assert_eq!(chain.check_chain(), BlockChainError::HashMismatch)
    }

    #[test]
    fn test_genesis_chain() {
        assert_eq!(BlockChain::new().check_chain(), BlockChainError::BlockChainOk);
    }

    #[test]
    fn test_add_block() {
        let mut chain = BlockChain::new();
        chain.add_block(Block::new(256));
        assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);
    }
}
