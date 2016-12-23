#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use tred::{Parse};

mod tred;

fn main() {
	let mut f = File::open("tred.trd").unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s).unwrap();
	
	let parse = Parse::new();
	println!{"{:?}", parse.parse(&s)};
}
