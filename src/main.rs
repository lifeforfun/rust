#[macro_use]
extern crate failure;

mod libs;
use libs::azul::test;

fn main() {
    test();
}
