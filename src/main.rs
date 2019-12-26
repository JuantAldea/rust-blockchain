pub mod block;
pub mod blockchain;
use block::*;
use blockchain::*;

fn main() {
    let block1 = Block::new(128);

    println!("{:?}", calculate_hash(&block1));

    let mut chain = BlockChain::new();
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());
}
