#![feature(box_syntax, box_patterns, slice_patterns)]

extern crate core;

extern crate aster;
#[cfg(feature = "nightly")]
extern crate syntax;
#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;
extern crate unescape;

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

fn main() {
    let mut f = File::open("src/parse.trd").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    match parse(&s) {
        Ok(toks) => compile::compile(&toks[..]),
        err => println!("{:?}", err)
    }
}
