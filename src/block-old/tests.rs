#[cfg(test)]
use super::*;
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
