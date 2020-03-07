#![allow(unused)]
//#[macro_use]
//extern crate failure;

pub mod libs;
//use libs::executor::test;
use libs::ffi::snappy::test;

fn main() {
    test();
}