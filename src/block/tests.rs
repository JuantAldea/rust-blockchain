#[cfg(test)]
use super::*;
#[test]
fn block_hash() {
    let block = Block {
        index: 1,
        previous_hash: GenericArray::default(),
        timestamp: 3,
        transaction: 4,
        proof: 5,
    };

    let block_hash = [
        215, 1, 133, 107, 24, 176, 226, 223, 99, 179, 8, 183, 126, 100, 144, 175, 167, 119, 166,
        90, 242, 241, 175, 21, 133, 88, 191, 223, 157, 136, 183, 245,
    ];

    let hash = block.hash();
    let hash: [u8; 32] = hash.into();
    assert_eq!(block_hash, hash);
}
