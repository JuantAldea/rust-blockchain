pub mod blockchain;

use blockchain::block::*;
use blockchain::chain::*;
use blockchain::transaction::*;
use blockchain::wallet::*;
use std::io::Write;
//use std::env;

fn main() {
    //env::set_var("RUST_LOG", "trace");

    env_logger::builder()
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    println!("==========================================================================");
    println!("=============================== WALLETS ==================================");
    println!("==========================================================================");
    let mut wallet1 = Wallet::new();
    let mut wallet2 = Wallet::new();
    println!("{}", wallet1);
    println!("{}", wallet2);

    let mut chain = BlockChain::new(3);
    println!("{}", chain);
    println!("{:#?}", chain.check_chain());

    println!("==========================================================================");
    println!("========================== CREATING GENESIS BLOCK #0 =====================");
    println!("==========================================================================");

    let tx = Transaction::new(
        0,
        // Funds out of nowhere! -> 0...0 <=> coinbase transaction
        &String::from("0").repeat(64),
        &wallet1.id.id,
        &wallet1.id.id,
        20,
    );

    let tx_signed = wallet1.sign_transaction(&tx);
    let block = Block::new(vec![tx_signed]);

    let mine_result = chain.mine_block(block);
    assert_eq!(mine_result, BlockChainOperationResult::BlockChainOk);

    let chain_check = chain.check_chain();
    assert_eq!(chain_check, BlockChainOperationResult::BlockChainOk);


    println!("=========================== Chain Updated ================================");
    println!("{}", chain);
    println!("==========================================================================");
    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("==========================================================================");
    println!("========================== BLOCK #1 ======================================");
    println!("==========================================================================");

    let tx = Transaction::new(
        0,
        &String::from("0").repeat(64),
        &wallet2.id.id,
        &wallet2.id.id,
        20,
    );

    let tx_signed = wallet2.sign_transaction(&tx);
    let block = Block::new(vec![tx_signed]);

    let mine_result = chain.mine_block(block);
    assert_eq!(mine_result, BlockChainOperationResult::BlockChainOk);

    let chain_check = chain.check_chain();
    assert_eq!(chain_check, BlockChainOperationResult::BlockChainOk);

    println!("=========================== Chain Updated ================================");
    println!("{}", chain);
    println!("==========================================================================");
    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("==========================================================================");
    println!("========================== BLOCK #2 ======================================");
    println!("==========================================================================");

    let transactions = wallet1.create_transaction(&wallet2.id, 7).unwrap();
    let block = Block::new(wallet1.sign_transactions(transactions));

    let mine_result = chain.mine_block(block);
    assert_eq!(mine_result, BlockChainOperationResult::BlockChainOk);

    let chain_check = chain.check_chain();
    assert_eq!(chain_check, BlockChainOperationResult::BlockChainOk);

    println!("=========================== Chain Updated ================================");
    println!("{}", chain);
    println!("==========================================================================");
    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("==========================================================================");
    println!("========================== BLOCK #3 ======================================");
    println!("==========================================================================");

    let transactions = wallet1.create_transaction(&wallet2.id, 2).unwrap();
    let block = Block::new(wallet1.sign_transactions(transactions));

    let mine_result = chain.mine_block(block);
    assert_eq!(mine_result, BlockChainOperationResult::BlockChainOk);

    let chain_check = chain.check_chain();
    assert_eq!(chain_check, BlockChainOperationResult::BlockChainOk);

    println!("=========================== Chain Updated ================================");
    println!("{}", chain);
    println!("==========================================================================");
    println!("=============================== Wallets ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("==========================================================================");
    println!("========================== BLOCK #4 ======================================");
    println!("==========================================================================");

    let transactions: Vec<Transaction> = vec![
        wallet2.create_transaction(&wallet1.id, 20).unwrap(),
        wallet2.create_transaction(&wallet1.id, 5).unwrap(),
    ]
    .iter()
    .flat_map(|vec| vec.iter())
    .cloned()
    .collect();

    let block = Block::new(wallet2.sign_transactions(transactions));

    let mine_result = chain.mine_block(block);
    assert_eq!(mine_result, BlockChainOperationResult::BlockChainOk);

    let chain_check = chain.check_chain();
    assert_eq!(chain_check, BlockChainOperationResult::BlockChainOk);

    println!("=========================== Chain Updated ================================");
    println!("{}", chain);
    println!("==========================================================================");
    println!("=============================== Wallets ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");
}
