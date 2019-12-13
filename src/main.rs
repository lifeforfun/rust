#[macro_use]
extern crate failure;

mod libs;
use libs::hero_story::hero::test;

fn main() {
    test();
}
