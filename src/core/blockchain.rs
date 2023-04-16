use super::block::*;
use crate::types::hash::Hash;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(genesis: Block) -> Self {
        let bc = Blockchain {
            blocks: vec![genesis],
        };
        bc
    }

    pub fn add_block(&mut self, mut block: Block) -> Result<(), String> {
        self.verify(&mut block)?;

        // execute transactions

        // add transaction
        self.blocks.push(block);
        Ok(())
    }

    pub fn verify(&mut self, block: &mut Block) -> Result<(), String> {
        if self.has_block(block.header.height) {
            return Err(format!(
                "chain already contains block with height {} => hash {}",
                block.header.height,
                self.get_block(block.header.height).unwrap().hash(),
            ));
        }

        if block.header.height != self.height() + 1 {
            return Err(format!(
                "block {} with height {} is too high => current height {}",
                block.hash(),
                block.header.height,
                self.height(),
            ));
        }

        let prev_header = self.get_header(block.header.height - 1).unwrap();
        let prev_hash = prev_header.hash();

        if block.header.prev_block_hash != prev_hash {
            return Err(format!(
                "the hash of the previous block {} is invalid",
                prev_hash
            ));
        }

        block.verify()
    }

    pub fn get_block(&mut self, height: u32) -> Result<&mut Block, String> {
        // TODO: add mutex
        if height > self.height() {
            return Err(format!("height {} too height", height));
        }
        Ok(&mut self.blocks[height as usize])
    }

    pub fn get_header(&mut self, height: u32) -> Result<&mut Header, String> {
        let block = self.get_block(height)?;
        return Ok(&mut block.header);
    }

    pub fn has_block(&self, height: u32) -> bool {
        height <= self.height()
    }

    pub fn height(&self) -> u32 {
        self.blocks.len() as u32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn new_blockchain_with_genesis() -> Blockchain {
        let block = random_block(0, "".to_string());
        Blockchain::new(block)
    }

    pub fn prev_block_hash(bc: &mut Blockchain, height: u32) -> Hash {
        let prev_block = bc.get_block(height - 1);
        prev_block.unwrap().header.hash()
    }

    #[test]
    fn test_add_block() {
        let mut bc = new_blockchain_with_genesis();
        let new_block = random_block(1, prev_block_hash(&mut bc, 1));
        let r = bc.add_block(new_block);
        println!("{:?}", r);
        println!("{}", r.is_ok());
        println!("{:?}", bc);
    }

    // fn test_verify_block {
    //     let block1 = random_block();
    //     let bc = new_blockchain(block);
    //     let new_block = random_block();
    //     new_block.header.prev_block_hash
    // }
}
