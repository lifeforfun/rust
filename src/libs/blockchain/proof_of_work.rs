use crate::libs::blockchain::block_chain::*;

const TARGET_BIT: u64 = 20;

struct ProofOfWork {
    pub block: Block,
    pub target: u64,
}

impl ProofOfWork {
    fn new(b: Block) -> Self {
        let target = 1u64;
        ProofOfWork{
            block: b,
            target: target << (256-TARGET_BIT),
        }
    }

    fn prepare_data(&mut self, nonce: i32) -> Vec<u8> {
        let mut v = vec![];
        v.append(&mut self.block.prev_block_hash.clone());
        v.append(&mut self.block.data.clone());
        v.append(&mut format!("{:x}", self.block.timestamp).as_bytes().to_vec());
        v.append(&mut format!("{:x}", TARGET_BIT).as_bytes().to_vec());
        v.append(&mut format!("{:x}", nonce).as_bytes().to_vec());
        v
    }
}
