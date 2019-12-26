pub mod block;
pub mod blockchain;
use block::*;
use blockchain::*;

fn main() {
    let mut chain = BlockChain::new();
    println!("Genesis Chain\n{:}", chain);
    println!("{:?}", chain.check_chain());

    let block = Block::new(128);

    chain.add_block(block);
    println!("{:}", chain);
    println!("{:?}", chain.check_chain());

    let block = Block::new(256);
    chain.add_block(block);
    println!("{:}", chain);
    println!("{:?}", chain.check_chain());

    let block = Block::new(512);
    chain.add_block(block);
    println!("{:}", chain);
    println!("{:?}", chain.check_chain());

    let block = Block::new(1024);
    chain.add_block(block);
    println!("{:}", chain);
    println!("{:?}", chain.check_chain());
}
