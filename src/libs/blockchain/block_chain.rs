extern crate crypto;

use std::time::SystemTime;
use crate::libs::blockchain::proof_of_work::ProofOfWork;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u32,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = std::str::from_utf8(&self.data[..]).unwrap();
        write!(f, "{}", s)
    }
}

impl Block {

    pub fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
        let block = Rc::new(Block{
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32,
            data,
            prev_block_hash,
            hash: vec![],
            nonce: 0,
        });
        let tuple;

        {
            // Rc::clone 会增加strong count，在try_unwrap时需要保证strong count==1，所以要手动管理作用域
            let mut pow = ProofOfWork::new(Rc::clone(&block));
            tuple = pow.run();
        }

        match Rc::try_unwrap(block) {
            Ok(mut b) => {
                b.nonce = tuple.0;
                b.hash  = tuple.1[..].to_vec();
                b
            },
            Err(_) => {
                panic!("unwrap block failed");
            },
        }
    }

}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain{
            blocks:vec![new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: Vec<u8>) -> &mut Self {
        let prev_block =  &self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push( new_block);
        self
    }
}

pub fn new_genesis_block() -> Block {
    Block::new("Genesis block".as_bytes().to_vec(), vec![])
}

pub fn test()
{
    use std::str::from_utf8;

    let mut bc = Blockchain::new();
    bc.add_block("data1".as_bytes().to_vec())
        .add_block("data2".as_bytes().to_vec())
    ;

    for block in bc.blocks {
        println!("Prev hash: {:?}", from_utf8(&block.prev_block_hash[..]));
        println!("Data : {:?}", from_utf8(&block.data[..]));
        println!("Hash: {:?}", from_utf8(&block.hash[..]));
    }
}