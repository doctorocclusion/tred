#![feature(box_syntax, box_patterns, slice_patterns)]

extern crate core;

extern crate unescape;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate tredlib;

#[macro_use]
pub extern crate lazy_static;

mod parse;
mod compile;

use parse::*;

use tredlib::{ParseErr};
use tredlib::gen;
use tredlib::regex::{Regex};

use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let fname = env::args().nth(1).expect("No input file given: tredc [tred source file]");
    let mut f = File::open(fname).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    match parse(&s) {
        Ok(toks) => compile::compile(&toks[..]),
        err => println!("{:?}", err)
    }
}
