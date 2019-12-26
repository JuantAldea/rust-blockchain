#[cfg(test)]
use super::*;
#[test]
fn block_hash() {
    let block = Block {
        index: 1,
        previous_hash: [0u8; 32],
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

    let block_hash = [
        150, 106, 200, 78, 143, 187, 171, 59, 57, 26, 131, 101, 66, 31, 195, 153, 129, 224, 214,
        190, 70, 120, 10, 110, 158, 92, 34, 91, 230, 169, 97, 5,
    ];

    let hash: [u8; 32] = block.hash().into();
    assert_eq!(block_hash, hash);
}
