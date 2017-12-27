#[macro_use]
extern crate tredlib;
#[macro_use]
pub extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use std::env;

mod json;

fn main() {
    let fname = env::args()
        .nth(1)
        .expect("No input file given: example [json file]");
    let mut f = File::open(fname).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("{:?}", json::parse(&s));
}
