#[macro_use]
extern crate failure;

mod libs;
use libs::json_parser::test;

fn main() {
    test();
}
