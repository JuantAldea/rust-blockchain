#[cfg(test)]
use super::block::*;
use super::transaction::*;
use super::chain::*;
use super::wallet::*;


#[test]
fn transaction_hash() {
    let mut tx1 = Transaction::new(
        0,
        &String::from("0").repeat(32),
        &String::from("1").repeat(32),
        &String::from("2").repeat(32),
        123,
    );

    tx1.timestamp = 1234;

    let transaction_hash = "d6ce8ef0a9b673007d04627f511a37d4f513263a35cf6df57cc8741ce8bb4643";
    assert_eq!(tx1.hash(), transaction_hash);
}

#[test]
fn signed_transaction_hash() {
    let mut tx1 = Transaction::new(
        0,
        &String::from("0").repeat(32),
        &String::from("1").repeat(32),
        &String::from("2").repeat(32),
        123,
    );

    tx1.timestamp = 1234;
    // no need for a valid signature here
    let mut signed_tx1 = SignedTransaction::new(&tx1);

    signed_tx1.timestamp = 12345;
    let uxto_hash = "67c0096516e871225ba31c709c4e521e0a6f0ad3e0914c625c0d98350f676fa3";
    assert_eq!(signed_tx1.uxto_hash(), uxto_hash);
}

/*
#[test]
fn signature() {

}
*/

#[test]
fn block_hash() {
    let mut tx1 = Transaction::new(
        0,
        &String::from("0").repeat(32),
        &String::from("1").repeat(32),
        &String::from("2").repeat(32),
        123,
    );

    let mut tx2 = Transaction::new(
        0,
        &String::from("0").repeat(32),
        &String::from("2").repeat(32),
        &String::from("3").repeat(32),
        123,
    );

    tx1.timestamp = 11111;
    tx2.timestamp = 22222;

    let mut signed_tx1 = SignedTransaction::new(&tx1);
    let mut signed_tx2 = SignedTransaction::new(&tx2);

    signed_tx1.timestamp = 33333;
    signed_tx2.timestamp = 44444;

    let block = Block {
        index: 1,
        previous_hash: String::from("0").repeat(64),
        timestamp: 3,
        transactions: vec![signed_tx1, signed_tx2],
        proof: 5,
    };

    let block_hash = "6fa9fc0f53a4b7a6c826e768f8bd544359f87f79aadadc42fc9d16a628bcd332";
    assert_eq!(block.hash(), block_hash);
}

#[test]
fn double_spend() {
    let mut wallet1 = Wallet::new();
    let mut wallet2 = Wallet::new();
    let tx1 = Transaction::new(
        0,
        &String::from("0").repeat(64),
        &wallet1.id.id,
        &wallet1.id.id,
        20,
    );

    let tx2 = Transaction::new(
        0,
        &String::from("0").repeat(64),
        &wallet2.id.id,
        &wallet2.id.id,
        20,
    );

    let tx1_signed = wallet1.sign_transaction(&tx1);
    let tx2_signed = wallet2.sign_transaction(&tx2);

    let mut chain = BlockChain::new(2, vec![tx1_signed, tx2_signed]);

    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);

    let transactions = wallet1.create_transaction(&wallet2.id, 7).unwrap();

    let block = wallet1.sign_transactions(transactions);
    assert_eq!(chain.validate_block(&block), BlockChainOperationResult::BlockChainOk);
    chain.add_block(block);

    // request another transaction without updating the wallets. It will pick the same UXTOs
    let transactions = wallet1.create_transaction(&wallet2.id, 2).unwrap();
    let block = wallet1.sign_transactions(transactions);
    // leading to a DoubleExpend
    assert_eq!(chain.validate_block(&block), BlockChainOperationResult::DoubleExpendingError);
}

#[test]
fn try_to_expend_uxtos_from_another() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let tx1 = Transaction::new(
        0,
        &String::from("0").repeat(64),
        &wallet1.id.id,
        &wallet1.id.id,
        20,
    );

    let tx2 = Transaction::new(
        0,
        &String::from("0").repeat(64),
        &wallet2.id.id,
        &wallet2.id.id,
        20,
    );

    let tx1_signed = wallet1.sign_transaction(&tx1);
    let tx2_signed = wallet2.sign_transaction(&tx2);
    let tx2_uxto = tx2_signed.uxto_hash.clone();
    let chain = BlockChain::new(2, vec![tx1_signed, tx2_signed]);

    //steal uxto from wallet2
    let bogus_tx = Transaction::new(
        0,
        &tx2_uxto,
        &wallet1.id.id,
        &wallet2.id.id,
        20,
    );

    let block = wallet1.sign_transactions(vec![bogus_tx]);
    assert_eq!(chain.validate_block(&block), BlockChainOperationResult::UXTOOwnershipError);
}

/*
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
*/
