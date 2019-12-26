#[cfg(test)]
use super::*;

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
    use generic_array::GenericArray;
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

    chain.chain[1].previous_hash = GenericArray::default();

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
    assert_eq!(
        BlockChain::new().check_chain(),
        BlockChainError::BlockChainOk
    );
}

#[test]
fn test_add_block() {
    let mut chain = BlockChain::new();
    chain.add_block(Block::new(256));
    assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);
}
