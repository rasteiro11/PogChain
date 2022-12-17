use block::block::Block;
use block_chain::block_chain::BlockChain;
use config::config::{Config, EnvConfig};
use crypto::crypto::SHA256;
use miner::miner::{IMiner, Miner};

use crate::crypto::crypto::Hasher;

mod block;
mod block_chain;
mod config;
mod crypto;
mod miner;

fn main() {
    // let hasher = SHA256::new();

    //for i in 0..100 {
    //    println!("{}", SHA256::hash(i.to_string()));
    //}

    let s = "test";
    let string = "test".to_string();

    println!("EQUAL: {}", s == string.get(0..).unwrap());

    let config = EnvConfig::new(".env").unwrap();
    println!("CONFIG OK");
    println!(
        "{}{}{}",
        config.get_reward(),
        config.get_difficulty(),
        config.get_genesis_prev_hash()
    );

    let block = Block::new(420, "MUITO CARO".into(), config.get_genesis_prev_hash());
    println!("BLOCK OK");
    let mut block_chain = BlockChain::new();
    println!("BLOCK CHAIN OK");

    let miner = Miner::new();
    println!("MINER OK");

    miner.mine(Box::new(block), &mut block_chain);
    //miner.is_golden_hash(&block);
}
