#[macro_use]
extern crate tredlib;
#[macro_use]
pub extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;

mod parse;

fn main() {
	let mut f = File::open("../test.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("{:?}", parse::parse(&s));
}
