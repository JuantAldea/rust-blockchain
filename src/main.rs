pub mod block;

fn main() {
    let block1 = block::Block::new(128);

    println!("{:?}", block::calculate_hash(&block1));

    let mut chain = block::BlockChain::new();
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    //chain.chain[1].proof = 0;
    //chain.chain[1].index = 0;
    //chain.chain[1].previous_hash = 0;
    //chain.chain[1].transaction = 2;

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());

    chain.add_block(block1);
    println!("{:?}", chain);
    println!("{:?}", chain.check_chain());
}
