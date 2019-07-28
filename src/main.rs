#[macro_use]
extern crate failure;

mod libs;
use libs::blockchain::test;

fn main() {
    test();
}
