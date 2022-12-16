use std::collections::LinkedList;

use crate::block::block::Block;

trait IBlockChain {
    fn add_block(&mut self, block: Block);
    fn get_size(&self) -> usize;
    fn get_block_chain(&self) -> &LinkedList<Block>;
}

pub struct BlockChain {
    // immutable ledger
    // we are not able to remove blocks
    block_chain: LinkedList<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        BlockChain {
            block_chain: LinkedList::new(),
        }
    }
}

impl IBlockChain for BlockChain {
    fn add_block(&mut self, block: Block) {
        self.block_chain.push_back(block);
    }

    fn get_size(&self) -> usize {
        self.block_chain.len()
    }

    fn get_block_chain(&self) -> &LinkedList<Block> {
        &self.block_chain
    }
}

impl ToString for dyn IBlockChain {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for block in self.get_block_chain() {
            s.push_str(block.to_string().as_str())
        }
        s
    }
}
