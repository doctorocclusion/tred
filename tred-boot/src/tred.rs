/*
* Hand compiled by Sam Sartor as part of the bootstrapping process
*/

/*
* THIS IS A GENERATED SOURCE FILE, COMPILED FROM tred.trd
* ALL CHANGES WILL BE OVERRIDEN
*
* Usage: 
*   1. Create a new parser using Parse::new()
*   2. Pass input text into the parse(&self, &str) function
*   3. Traverse the outputted Item tree
* 
* Note that a only a single Parse object needs to be created.
*
* Also note that the regex create is required
*/

use regex::Regex;

#[derive(Debug)]
pub enum Item {
    Expr(Box<Item>, Vec<Box<Item>>),
    Comment(String),
    Name(String),
    Tuple(Box<Item>, Vec<Box<Item>>),
    Regex(String),
    StrLiteral(String),
    Block(Vec<Box<Item>>)
}

#[derive(Debug)]
pub struct ParseErr {
    at: usize
}

pub struct Parse {
    white_regex: Regex,
    blank_regex: Regex,
    name_regex: Regex,
    comment_regex: Regex,
}

impl Parse {
    pub fn parse(&self, input: &str) -> Result<(usize, Vec<Box<Item>>), ParseErr> {
        let res = try!(block_main(self, input, 0, vec![]));
        Ok((res.1, res.2))
    }

    pub fn new() -> Parse {
        Parse {
            white_regex: Regex::new(r"^[\s\n\r]*").unwrap(),
            blank_regex: Regex::new(r"^[\s\n\r]+").unwrap(),
            name_regex: Regex::new(r"^[\w_]+").unwrap(),
            comment_regex: Regex::new(r"^[^\n]*").unwrap(),
        }
    }
}

type BlockFn<'a, 'b> = fn(&'b Parse, &'a str, usize, Vec<Box<Object>>) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr>;

enum Object<'a, 'b> {
    Str(String),
    Name(String),
    Reg(Regex),
    Block(BlockFn<'a, 'b>),
    It(Item)
}

fn block_line_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize, pars: Vec<Box<Object>>) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    if input.len() < 5 { return Err(ParseErr{at: pos}); }
    Ok((&input[5..], 1usize, vec![Box::new(Item::Comment(input[..5].to_string()))])) 
}

fn block_main<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize, pars: Vec<Box<Object>>) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut into = vec![];
    let mut text = input;
    let mut total = 0usize;
    loop {
        let res = block_line_m(parse, text, pos + total, vec![]);
        if let Ok(mut x) = res {
            text = x.0;
            total += x.1;
            into.append(&mut x.2);
        } else { break; }
    }
    Ok((text, total, into))
}