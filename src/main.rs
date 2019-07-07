#[macro_use] extern crate failure;

mod libs;
use libs::testtokio::timer::test;

fn main()
{
    test();
}