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

use tredlib::{ParseErr};
use tredlib::gen;
use tredlib::regex::{Regex};

use parse::{Parse};

use std::io::prelude::*;
use std::fs::File;

lazy_static! {
    static ref TRED_PARSER: Parse = Parse::new();
}

fn main() {
    let mut f = File::open("../json.trd").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    match TRED_PARSER.parse(&s) {
        Ok((_, toks)) => compile::compile(&toks[..]),
        err => println!("{:?}", err)
    }
}
