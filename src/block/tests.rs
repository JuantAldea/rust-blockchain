#[cfg(test)]
use super::*;
#[test]
fn block_hash() {
    let block = Block {
        index: 1,
        previous_hash: String::from("0").repeat(64),
        timestamp: 3,
        transactions: vec![
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
        proof: 5,
    };

    let block_hash = "541cecc1c6af2ecb4f7b9a7e1353c9cebf02490d121e319df5c02d9a98938f99";
    assert_eq!(block_hash, block.hash());
}
