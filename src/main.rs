pub mod block;
pub mod blockchain;
use block::*;
use blockchain::*;

fn main() {
    let mut chain = BlockChain::new(vec![Transaction {
        sender: 0,
        recipient: 0,
        amount: 0,
    }]);
    println!("{}\n", chain);
    println!("{:?}\n", chain.check_chain());

    let block = Block::new(vec![Transaction {
        sender: 128,
        recipient: 0,
        amount: 0,
    }]);

    chain.add_block(block);
    println!("\n{}\n", chain);
    println!("{:?}\n", chain.check_chain());

    let block = Block::new(vec![Transaction {
        sender: 256,
        recipient: 0,
        amount: 0,
    }]);

    chain.add_block(block);
    println!("\n{}\n", chain);
    println!("{:?}\n", chain.check_chain());

    let block = Block::new(vec![Transaction {
        sender: 512,
        recipient: 0,
        amount: 0,
    }]);

    chain.add_block(block);
    println!("\n{}\n", chain);
    println!("{:?}\n", chain.check_chain());

    let block = Block::new(vec![
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
    ]);

    chain.add_block(block);
    println!("\n{}\n", chain);
    println!("{:?}\n", chain.check_chain());
}
