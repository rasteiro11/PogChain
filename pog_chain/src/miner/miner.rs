use std::borrow::Borrow;

use crate::{
    block::block::{Block, IBlock},
    block_chain::block_chain::IBlockChain,
    config::config::{Config, EnvConfig},
};

pub trait IMiner {
    fn get_reward(&self) -> u64;
    fn mine(&self, block: Box<dyn IBlock>, block_chain: &mut dyn IBlockChain);
    fn is_golden_hash(&self, block: &dyn IBlock) -> bool;
}

pub struct Miner {
    reward: u64,
    config: Box<dyn Config>,
}

impl Miner {
    pub fn new() -> Self {
        let config = EnvConfig::new(".env").unwrap();
        Miner {
            reward: config.get_reward(),
            config: Box::new(config),
        }
    }
}

impl IMiner for Miner {
    fn get_reward(&self) -> u64 {
        return self.reward;
    }

    fn mine(&self, mut block: Box<dyn IBlock>, block_chain: &mut dyn IBlockChain) {
        while !self.is_golden_hash(block.borrow()) {
            block.increment_nonce();
            block.generate_hash();
        }

        println!("{} has just mined...", block.get_id());
        println!("Hash is {}", block.get_hash());

        block_chain.add_block(block);
    }

    fn is_golden_hash(&self, block: &dyn IBlock) -> bool {
        let mut leading_zeros = String::new();
        for _ in 0..self.config.get_difficulty() {
            leading_zeros.push('0');
        }

        block.get_hash().starts_with(leading_zeros.as_str())
    }
}
