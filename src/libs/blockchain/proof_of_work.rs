use crate::libs::blockchain::block_chain::*;

const TARGET_BIT: u8 = 20;

struct ProofOfWork {
    block: Block,
    target: u64,
}

impl ProofOfWork {
    fn new(b: Block) -> Self {
        let mut target = 1;

    }
}
