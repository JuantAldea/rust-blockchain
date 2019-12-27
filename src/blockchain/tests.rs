use super::block::*;
use super::chain::*;
use super::transaction::*;

#[cfg(test)]
#[test]
fn block_hash() {
    let block = Block {
        index: 1,
        previous_hash: String::from("0").repeat(64),
        timestamp: 3,
        transactions: vec![
            Transaction::new(0, &String::from(""), 128, 0, 0),
            Transaction::new(0, &String::from(""), 128, 0, 0),
        ],
        proof: 5,
    };

    let block_hash = "f05b86b4d63197329b099f9560dcec94e69680c699ed6cfedd07fedd36b10d64";
    assert_eq!(block.hash(), block_hash);
}

#[allow(dead_code)]
fn generate_chain() -> BlockChain {
    let mut chain = BlockChain::new(
        4,
        vec![
            Transaction::new(0, &String::from(""), 128, 0, 0),
            Transaction::new(0, &String::from(""), 128, 0, 0),
        ],
    );

    chain.chain[0].timestamp = 0;

    let mut block = Block::new(vec![
        Transaction::new(0, &String::from(""), 128, 0, 0),
        Transaction::new(0, &String::from(""), 128, 0, 0),
    ]);

    block.timestamp = 1;
    chain.add_block(block);

    let mut block = Block::new(vec![
        Transaction::new(0, &String::from(""), 128, 0, 0),
        Transaction::new(0, &String::from(""), 128, 0, 0),
    ]);

    block.timestamp = 2;
    chain.add_block(block);

    let mut block = Block::new(vec![
        Transaction::new(0, &String::from(""), 128, 0, 0),
        Transaction::new(0, &String::from(""), 128, 0, 0),
    ]);

    block.timestamp = 3;
    chain.add_block(block);
    chain
}

#[test]
fn test_chain2() {
    let chain = generate_chain();

    println!("{}", chain);
    println!("{:?}", chain.check_chain());
    println!("{}", chain.get_last_hash());
    assert_eq!(
        chain.get_last_hash(),
        "1e00ad033a46b301e570af2cec3c677a24d03f0e2501d95146f4c8f6176aad9a"
    );
    assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);
}

#[test]
fn test_chain_break_proof() {
    let mut chain = generate_chain();
    chain.chain[1].proof = 0;

    println!("{}", chain);
    println!("{:?}", chain.check_chain());
    assert_eq!(chain.check_chain(), BlockChainError::ProofOfWorkError);
}

#[test]
fn test_chain_break_index() {
    let mut chain = generate_chain();
    chain.chain[2].index = 1;

    println!("{}", chain);
    println!("{:?}", chain.check_chain());
    assert_eq!(chain.check_chain(), BlockChainError::IndexMismatch);
}

#[test]
fn test_chain_break_previous_hash() {
    let mut chain = generate_chain();
    chain.chain[1].previous_hash = String::from("0").repeat(64);

    println!("{}", chain);
    println!("{:?}", chain.check_chain());
    assert_eq!(chain.check_chain(), BlockChainError::HashMismatch);
}

#[test]
fn test_chain_break_transaction() {
    let mut chain = generate_chain();
    chain.chain[1].transactions = vec![
        Transaction::new(0, &String::from("FAIL"), 128, 0, 0),
        Transaction::new(0, &String::from("FAIL"), 128, 0, 0),
    ];

    println!("{}", chain);
    println!("{:?}", chain.check_chain());
    assert_eq!(chain.check_chain(), BlockChainError::HashMismatch);
}

#[test]
fn test_genesis_chain() {
    assert_eq!(
        BlockChain::new(
            4,
            vec![
                Transaction::new(0, &String::from(""), 128, 0, 0,),
                Transaction::new(0, &String::from(""), 128, 0, 0,),
            ]
        )
        .check_chain(),
        BlockChainError::BlockChainOk
    );
}

#[test]
fn test_add_block() {
    let mut chain = BlockChain::new(
        4,
        vec![
            Transaction::new(0, &String::from(""), 128, 0, 0),
            Transaction::new(0, &String::from(""), 128, 0, 0),
        ],
    );

    chain.add_block(Block::new(vec![
        Transaction::new(0, &String::from(""), 128, 0, 0),
        Transaction::new(0, &String::from(""), 128, 0, 0),
    ]));

    assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);
}
