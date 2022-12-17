use std::{borrow::Borrow, collections::LinkedList};

use crate::block::block::IBlock;

pub trait IBlockChain {
    fn add_block(&mut self, block: Box<dyn IBlock>);
    fn get_block_chain(&self) -> &LinkedList<Box<dyn IBlock>>;
    fn get_size(&self) -> usize;
}

pub struct BlockChain {
    // immutable ledger
    // we are not able to remove blocks
    block_chain: LinkedList<Box<dyn IBlock>>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            block_chain: LinkedList::new(),
        }
    }
}

impl IBlockChain for BlockChain {
    fn add_block(&mut self, block: Box<dyn IBlock>) {
        self.block_chain.push_back(block);
    }

    fn get_size(&self) -> usize {
        self.block_chain.len()
    }

    fn get_block_chain(&self) -> &LinkedList<Box<dyn IBlock>> {
        &self.block_chain
    }
}

impl ToString for dyn IBlockChain {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for block in self.get_block_chain() {
            s.push_str(block.to_string().clone().as_str());
            s.push_str("\n");
        }
        s
    }
}
