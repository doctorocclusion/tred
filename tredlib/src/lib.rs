pub extern crate regex;

pub mod gen;

#[derive(Debug)]
pub struct ParseErr {
    pub at: usize
}