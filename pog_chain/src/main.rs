use block::block::Block;
use block_chain::block_chain::BlockChain;
use config::config::{Config, EnvConfig};
use crypto::crypto::SHA256;
use miner::miner::{IMiner, Miner};

use crate::{block_chain::block_chain::IBlockChain, crypto::crypto::Hasher};

mod block;
mod block_chain;
mod config;
mod crypto;
mod miner;

fn main() {
    let config = EnvConfig::new(".env").unwrap();
    println!(
        "{}{}{}",
        config.get_reward(),
        config.get_difficulty(),
        config.get_genesis_prev_hash()
    );

    let mut block_chain = BlockChain::new();
    let mut miner = Miner::new();

    let block0 = Block::new(0, "".into(), config.get_genesis_prev_hash());
    miner.mine(Box::new(block0), &mut block_chain);

    let block1 = Block::new(
        1,
        "transaction1".into(),
        block_chain.get_block_chain().back().unwrap().get_hash(),
    );
    miner.mine(Box::new(block1), &mut block_chain);

    let block2 = Block::new(
        2,
        "transaction2".into(),
        block_chain.get_block_chain().back().unwrap().get_hash(),
    );
    miner.mine(Box::new(block2), &mut block_chain);

    println!(
        "BLOCKCHAIN {}",
        (&block_chain as &dyn IBlockChain).to_string()
    );

    println!("REWARD: {}", miner.get_reward());
}
