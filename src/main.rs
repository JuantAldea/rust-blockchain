pub mod blockchain;

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

    println!("========================== CREATING GENESIS BLOCK ========================");
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

    let mut chain = BlockChain::new(2, vec![tx1_signed, tx12_signed, tx2_signed]);

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
    let block = wallet1.sign_transactions(transactions);
    //println!("New block: {}\n", block);

    if chain.validate_block(&block) == BlockChainOperationResult::BlockChainOk {
        chain.add_block(block);
        println!("=========================== Chain Updated ================================");
        println!("{}", chain);
        println!("==========================================================================");
    }

    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    /************************************ */
    let transactions = wallet1.create_transaction(&wallet2.id, 2).unwrap();

    //println!("New Transactions: {:?}\n", transactions);
    let block = wallet1.sign_transactions(transactions);
    //println!("New block: {}\n", block);

    if chain.validate_block(&block) == BlockChainOperationResult::BlockChainOk {
        chain.add_block(block);
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

    /*
    let block = wallet1.create_transaction(2, 5).unwrap();
    println!("New block: {}\n", block);
    chain.add_block(block);

    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    let block = wallet2.create_transaction(1, 23).unwrap();
    println!("New block: {}\n", block);
    chain.add_block(block);
    println!("{}", chain);

    println!("=============================== WALLETS ==================================");
    wallet1.read_wallet(&chain);
    wallet2.read_wallet(&chain);
    println!("{}", wallet1);
    println!("{}", wallet2);
    println!("==========================================================================");

    println!("{:?}\n", chain.check_chain());
    */
}
