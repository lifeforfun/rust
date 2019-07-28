#[macro_use]
extern crate failure;

mod libs;
use libs::send_sync::test;

fn main() {
    test();
}
