pub extern crate regex;
#[macro_use]
pub extern crate lazy_static;

pub mod gen;

#[derive(Debug)]
pub struct ParseErr {
    pub at: usize
}