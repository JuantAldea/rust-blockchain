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
        25, 137, 162, 228, 182, 79, 0, 141, 6, 93, 164, 76, 112, 141, 248, 95, 66, 140, 70, 126,
        12, 85, 25, 177, 130, 74, 157, 117, 44, 56, 212, 140,
    ];

    let hash: [u8; 32] = block.hash().into();
    assert_eq!(block_hash, hash);
}
