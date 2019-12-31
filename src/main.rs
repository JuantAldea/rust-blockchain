pub mod blockchain;

use blockchain::block::*;
use blockchain::chain::*;
use blockchain::transaction::*;
use blockchain::wallet::*;
use std::io::Write;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    println!("=============================== WALLETS ==================================");
    let mut wallet1 = Wallet::new();
    let mut wallet2 = Wallet::new();
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("========================== CREATING EMPTY CHAIN ==========================");
    let mut chain = BlockChain::new(3);
    println!("{}", chain);
    println!("{:#?}", chain.check_chain());
    println!("==========================================================================");

    println!("========================== CREATING GENESIS BLOCK ========================");

    // Funds out of nowhere!
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
    let tx12_signed = wallet1.sign_transaction(&tx1);
    let transactions = vec![tx1_signed, tx12_signed, tx2_signed];
    let genesis_block = Block::new(transactions);

    chain.mine_block(genesis_block);
    println!("{}", chain);
    println!("{:#?}", chain.check_chain());
    println!("==========================================================================");

    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    let transactions = wallet1.create_transaction(&wallet2.id, 7).unwrap();

    //println!("New Transactions: {:?}\n", transactions);
    let block = Block::new(wallet1.sign_transactions(transactions));
    //println!("New block: {}\n", block);

    if chain.validate_block(&block) == BlockChainOperationResult::BlockChainOk {
        chain.mine_block(block);
        println!("=========================== Chain Updated ================================");
        println!("{}", chain);
        println!("==========================================================================");
    }
    println!("{:#?}", chain.check_chain());

    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    /************************************ */
    let transactions = wallet1.create_transaction(&wallet2.id, 2).unwrap();

    //println!("New Transactions: {:?}\n", transactions);
    let block = Block::new(wallet1.sign_transactions(transactions));
    //println!("New block: {}\n", block);

    if chain.validate_block(&block) == BlockChainOperationResult::BlockChainOk {
        chain.mine_block(block);
        println!("=========================== Chain Updated ================================");
        println!("{}", chain);
        println!("==========================================================================");
    }

    println!("=============================== Wallets ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    let transactions: Vec<Transaction> = vec![
        wallet2.create_transaction(&wallet1.id, 20).unwrap(),
        wallet2.create_transaction(&wallet1.id, 5).unwrap(),
    ]
    .iter()
    .flat_map(|vec| vec.iter())
    .cloned()
    .collect();

    println!("{}", TransactionsVec(transactions.clone()));

    let block = Block::new(wallet2.sign_transactions(transactions));
    if chain.validate_block(&block) == BlockChainOperationResult::BlockChainOk {
        chain.mine_block(block);
        println!("=========================== Chain Updated ================================");
        println!("{}", chain);
        println!("==========================================================================");
    }

}
