extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

#[macro_use]
extern crate tredlib;

use tredlib::{ParseErr};
use tredlib::gen;
use tredlib::regex::{Regex};

fn op1(p:usize, s: &str) -> Result<(&str, usize, Vec<i32>), ParseErr> {
	let mut pos:usize = p;
    let mut text = &s[..];
    let mut out = vec![];

    _gen_match_str!(pos, text, out, "l");
    out.push(1);

    Ok((text, pos, out))
}

#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_mut)]
fn op2(p:usize, s: &str) -> Result<(&str, usize, Vec<i32>), ParseErr> {
	let mut pos:usize = p;
    let mut text = &s[..];
    let mut out = vec![];

    _gen_match_regex!(pos, text, out, Regex::new(r"[^l]+").unwrap());
    out.push(2);

    Ok((text, pos, out))
}

fn op3(p:usize, s: &str) -> Result<(&str, usize, Vec<i32>), ParseErr> {
	let mut pos:usize = p;
    let mut text = &s[..];
    let mut out = vec![];

    _gen_match_str!(pos, text, out, " ");
    out.push(3);

    Ok((text, pos, out))
}

fn block_2(p:usize, s: &str) -> Result<(&str, usize, Vec<i32>), ParseErr> {
    let mut pos:usize = p;
    let mut text = &s[..];
    let mut out = vec![];

	_gen_or!(pos, text, out, op1(pos, text), op3(pos, text), op2(pos, text));

    Ok((text, pos, out))
}

fn block_ex(s: &str) -> Result<(&str, usize, Vec<i32>), ParseErr> {
    let mut pos:usize = 0;
    let mut text = &s[..];
    let mut out = vec![];

    _gen_many!(pos, text, out, block_2(pos, text));

    Ok((text, pos, out))
}

fn main() {
	println!("{:?}", block_ex(&"   hello"[..]));
}
