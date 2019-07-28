#[macro_use]
extern crate failure;

mod libs;
use libs::block_chain::test;

fn main() {
    test();
}
