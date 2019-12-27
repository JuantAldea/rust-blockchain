#[cfg(test)]
use super::Block;
use super::BlockChain;
use super::Transaction;
use super::*;

#[allow(dead_code)]
fn generate_chain() -> BlockChain {
    let mut chain = BlockChain::new(
        4,
        vec![
            Transaction {
                sender: 1024,
                recipient: 0,
                amount: 0,
            },
            Transaction {
                sender: 2048,
                recipient: 0,
                amount: 0,
            },
        ],
    );
    chain.chain[0].timestamp = 0;

    let mut block = Block::new(vec![
        Transaction {
            sender: 1024,
            recipient: 0,
            amount: 0,
        },
        Transaction {
            sender: 2048,
            recipient: 0,
            amount: 0,
        },
    ]);

    block.timestamp = 1;
    chain.add_block(block);

    let mut block = Block::new(vec![
        Transaction {
            sender: 1024,
            recipient: 0,
            amount: 0,
        },
        Transaction {
            sender: 2048,
            recipient: 0,
            amount: 0,
        },
    ]);

    block.timestamp = 2;
    chain.add_block(block);

    let mut block = Block::new(vec![
        Transaction {
            sender: 4096,
            recipient: 0,
            amount: 0,
        },
        Transaction {
            sender: 8192,
            recipient: 0,
            amount: 0,
        },
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
    assert_eq!(chain.get_last_hash(), "8505d00d102d65b9f473d64e1ee1602fad7f7c4115a6385e9b8c6bde128ee24b");
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
        Transaction {
            sender: 2000,
            recipient: 0,
            amount: 0,
        },
        Transaction {
            sender: 2048,
            recipient: 0,
            amount: 0,
        },
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
                Transaction {
                    sender: 1024,
                    recipient: 0,
                    amount: 0,
                },
                Transaction {
                    sender: 2048,
                    recipient: 0,
                    amount: 0,
                },
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
            Transaction {
                sender: 1024,
                recipient: 0,
                amount: 0,
            },
            Transaction {
                sender: 2048,
                recipient: 0,
                amount: 0,
            },
        ],
    );

    chain.add_block(Block::new(vec![
        Transaction {
            sender: 1024,
            recipient: 0,
            amount: 0,
        },
        Transaction {
            sender: 2048,
            recipient: 0,
            amount: 0,
        },
    ]));

    assert_eq!(chain.check_chain(), BlockChainError::BlockChainOk);
}
