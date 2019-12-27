pub mod blockchain;

use blockchain::chain::*;
use blockchain::transaction::*;
use blockchain::wallet::*;

fn main() {
    let mut chain = BlockChain::new(
        4,
        vec![
            Transaction::new(0, &String::from("0").repeat(64), 0, 1, 20),
            Transaction::new(0, &String::from("0").repeat(64), 0, 2, 20),
            //Transaction::new(0, &String::from("0").repeat(64), 0, 3, 20),
        ],
    );

    println!("{}", chain);
    println!("==========================================================================");
    let wallet1 = Wallet::new(&chain, 1);
    let wallet2 = Wallet::new(&chain, 2);
    //let wallet3 = Wallet::new(&chain, 3);

    println!("Wallet1 {:}", wallet1);
    println!("Wallet2 {:}", wallet2);
    println!("==========================================================================");

    //println!("{:}", wallet3);

    let block = wallet1.create_transaction(2, 15).unwrap();

    println!("New block: {}\n", block);
    chain.add_block(block);
    println!("==========================================================================");
    println!("New chain:\n{}\n", chain);
    println!("==========================================================================");
    //println!("{:?}\n", chain.check_chain());

    let wallet1 = Wallet::new(&chain, 1);
    let wallet2 = Wallet::new(&chain, 2);

    println!("Wallet1:\n{:}", wallet1);
    println!("Wallet2:\n{:}", wallet2);

    let block = wallet1.create_transaction(2, 5).unwrap();

    println!("New block: {}\n", block);
    chain.add_block(block);

    println!("==========================================================================");
    let wallet1 = Wallet::new(&chain, 1);
    let wallet2 = Wallet::new(&chain, 2);

    println!("Wallet1:\n{:}", wallet1);
    println!("Wallet2:\n{:}", wallet2);
    println!("==========================================================================");

    let block = wallet2.create_transaction(1, 23).unwrap();

    println!("New block: {}\n", block);
    chain.add_block(block);
    println!("{}", chain);
    println!("==========================================================================");
    let wallet1 = Wallet::new(&chain, 1);
    let wallet2 = Wallet::new(&chain, 2);

    println!("Wallet1:\n{:}", wallet1);
    println!("Wallet2:\n{:}", wallet2);

    println!("==========================================================================");
    println!("{:?}\n", chain.check_chain());
}
