#[cfg(test)]
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
