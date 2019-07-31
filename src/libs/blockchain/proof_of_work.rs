extern crate crypto;

use crate::libs::blockchain::block_chain::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

const TARGET_BIT: u64 = 20;

struct ProofOfWork {
    pub block: Block,
    pub target: u64,
}

fn copy_from_slice(&s: &[u8]) -> [u8; 8]
{
    let mut arr = [0; 8];
    let counter = 0;
    for i in s.iter() {
        if counter>=arr.len() {
            break;
        }
        arr[counter] = i.clone();
        counter += 1;
    }
    arr
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

    fn run(&mut self) -> (i32, Vec<u8>) {
        let hash_int;
        let hash;
        let nonce = 0;
        println!("Mining the block containing \"{:?}\"", self.block.data);

        loop {
            if nonce>=i32::max_value() {
                break;
            }
            let data = self.prepare_data(nonce);
            let mut hasher = Sha3::sha3_256();
            hasher.input(&data[..]);
            hash_int = u64::from_be_bytes(copy_from_slice(hasher.result_str().as_bytes()));

            if hash_int<self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }
}
