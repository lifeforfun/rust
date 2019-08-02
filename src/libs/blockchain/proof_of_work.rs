extern crate crypto;

use crate::libs::blockchain::block_chain::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::rc::Rc;
use rug::{Integer, Assign};

pub const TARGET_BIT: u32 = 20;

#[derive(Debug)]
pub struct ProofOfWork {
    pub block: Rc<Block>,
    pub target: Integer,
}

fn copy_from_slice(s: Vec<u8>) -> [u8; 8]
{
    let mut arr = [0; 8];
    let mut counter = 0;
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
    pub fn new(b: Rc<Block>) -> Self {
        let mut target = Integer::with_capacity(256);
        target.assign(1);
        ProofOfWork{
            block: b,
            target: target << (256-TARGET_BIT),
        }
    }

    pub fn prepare_data(&mut self, nonce: u64) -> Vec<u8> {
        let mut v = vec![];
        v.append(&mut self.block.prev_block_hash.clone());
        v.append(&mut self.block.data.clone());
        v.append(&mut format!("{:x}", self.block.timestamp).as_bytes().to_vec());
        v.append(&mut format!("{:x}", TARGET_BIT).as_bytes().to_vec());
        v.append(&mut format!("{:x}", nonce).as_bytes().to_vec());
        v
    }

    pub fn run(&mut self) -> (u64, Vec<u8>) {
        let mut hash_int;
        let mut hash = vec![0u8];
        let mut nonce = 0;
        println!("Mining the block containing \"{:?}\"", self.block.data);

        loop {
            if nonce>=u64::max_value() {
                break;
            }
            let data = self.prepare_data(nonce);
            let mut hasher = Sha3::sha3_256();
            hasher.input(&data[..]);
            hash = hasher.result_str().as_bytes().to_vec();
            hash_int = u64::from_be_bytes(copy_from_slice(hash.clone()));

            if hash_int<self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }
}
