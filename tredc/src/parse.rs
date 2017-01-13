/*
* Hand compiled by Sam Sartor as part of the bootstrapping process
*/

/*
* THIS IS A GENERATED SOURCE FILE, COMPILED FROM parse.trd
* ALL CHANGES WILL BE OVERRIDEN
*
* Usage: 
*   1. Create a new parser using Parse::new()
*   2. Pass input text into the parse(&self, &str) function
*   3. Traverse the outputSted Item tree
* 
* Note that a only a single Parse object needs to be created.
*
* Also note that the regex create is required
*/

use tredlib::{ParseErr};
use tredlib::regex::Regex;

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

pub struct Parse {
    white_regex: Regex,
    blank_regex: Regex,
    name_regex: Regex,
    comment_regex_1: Regex,
    comment_regex_2: Regex,
    strlit_regex: Regex,
    regex_regex: Regex
}

impl Parse {
    pub fn parse(&self, input: &str) -> Result<(usize, Vec<Box<Item>>), ParseErr> {
        let res = try!(block_main(self, input, 0));
        Ok((res.1, res.2))
    }

    pub fn new() -> Parse {
        Parse {
            white_regex: Regex::new(r"^[\s\n\r]*").unwrap(),
            blank_regex: Regex::new(r"^[\s\n\r]+").unwrap(),
            name_regex: Regex::new(r"^[\w_]+").unwrap(),
            comment_regex_1: Regex::new(r"^\s*").unwrap(),
            comment_regex_2: Regex::new(r"^[^\n]*").unwrap(),
            strlit_regex: Regex::new("^[^\"]+").unwrap(),
            regex_regex: Regex::new("^[^/]+").unwrap()
        }
    }
}

fn block_blank_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    if let Some((_, end)) = parse.blank_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        return Err(ParseErr{at: at + pos});
    }

    Ok((text, at, vec![]))
}

fn block_white_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    if let Some((_, end)) = parse.white_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        return Err(ParseErr{at: at + pos});
    }

    Ok((text, at, vec![]))
}

fn block_comment_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;
    
    if !text.starts_with("//") { return Err(ParseErr{at: at + pos}); }
    at += 2;
    text = &text[2..];

    if let Some((_, end)) = parse.comment_regex_1.find(text) {
        at += end;
        text = &text[end..];
    } else {
        return Err(ParseErr{at: at + pos});
    }

    let cap_start = at;

    if let Some((_, end)) = parse.comment_regex_2.find(text) {
        at += end;
        text = &text[end..];
    } else {
        return Err(ParseErr{at: at + pos});
    }

    let out = vec![Box::new(Item::Comment(input[cap_start..at].to_string()))];

    Ok((text, at, out))
}

fn block_strlit_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    if !text.starts_with("\"") {
        //println!("String Start Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];
    
    let cap_start = at;

    if let Some((_, end)) = parse.strlit_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        //println!("String Error: {:?}", at + pos);
        return Err(ParseErr{at: at + pos});
    }

    let out = vec![Box::new(Item::StrLiteral(input[cap_start..at].to_string()))];

    if !text.starts_with("\"") {
        //println!("String End Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    Ok((text, at, out))
}

fn block_regex_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    if !text.starts_with("/") {
        //println!("Regex Start Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];
    
    let cap_start = at;

    if let Some((_, end)) = parse.regex_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        //println!("Regex Error: {:?}", at + pos);
        return Err(ParseErr{at: at + pos});
    }

    let out = vec![Box::new(Item::Regex(input[cap_start..at].to_string()))];

    if !text.starts_with("/") {
        //println!("Regex End Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    Ok((text, at, out))
}

fn block_tuple_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    let cap_start = at;
    if let Some((_, end)) = parse.name_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        //println!("Tuple Name Error: {:?}", at + pos);
        return Err(ParseErr{at: at + pos});
    }
    let name = Box::new(Item::Name(input[cap_start..at].to_string()));

    if !text.starts_with("(") {
        //println!("Tuple Start Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    let mut args = vec![];
    let mut first = true;
    loop {
        if !first {
            let res = block_blank_m(parse, text, pos + at);
            if let Ok(mut x) = res {
                text = x.0;
                at += x.1;
                args.append(&mut x.2);
            } else {
                //println!("Tuple Arg Blank Error: {:?}", res);
                break; 
            }
        } else { first = false; }
        let res = block_value_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            args.append(&mut x.2);
        } else {
            //println!("Tuple Arg Error: {:?}", res);
            break;
        }
    }

    if !text.starts_with(")") {
        //println!("Tuple End Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    Ok((text, at, vec![Box::new(Item::Tuple(name, args))]))
}

fn block_name_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    let cap_start = at;
    if let Some((_, end)) = parse.name_regex.find(text) {
        at += end;
        text = &text[end..];
    } else {
        //println!("Name Error: {:?}", at + pos);
        return Err(ParseErr{at: at + pos});
    }

    let name = Box::new(Item::Name(input[cap_start..at].to_string()));

    Ok((text, at, vec![name]))
}

fn block_block_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;
    let mut out = vec![];

    if !text.starts_with("{") {
        //println!("Block Start Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    loop {
        let res = block_line_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            out.append(&mut x.2);
        } else {
            //println!("Block Line Error: {:?}", res);
            break; 
        }
    }

    if !text.starts_with("}") {
        //println!("Block End Error: {:?}", pos + at);
        return Err(ParseErr{at: at + pos}); 
    }
    at += 1;
    text = &text[1..];

    Ok((text, at, vec![Box::new(Item::Block(out))]))
}

fn block_value_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    if let Ok(res) = block_strlit_m(parse, input, pos) { return Ok(res); }
    if let Ok(res) = block_regex_m(parse, input, pos) { return Ok(res); } 
    if let Ok(res) = block_block_m(parse, input, pos) { return Ok(res); }
    if let Ok(res) = block_tuple_m(parse, input, pos) { return Ok(res); }
    if let Ok(res) = block_name_m(parse, input, pos) { return Ok(res); } 
    Err(ParseErr{at: pos})
}

fn block_exp_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut at = 0;
    let mut text = input;

    let mut op;
    {
        let res = block_value_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            op = x.2.pop().unwrap();
        } else {
            //println!("Exp Op Error: {:?}", res);
            return res; 
        }
    }

    let mut args = vec![];
    loop {
        let res = block_blank_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            args.append(&mut x.2);
        } else {
            //println!("Exp Arg Blank Error: {:?}", res);
            break; 
        }
        let res = block_value_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            args.append(&mut x.2);
        } else {
            //println!("Exp Arg Error: {:?}", res);
            break;
        }
    }

    {
        let res = block_white_m(parse, text, pos + at);
        if let Ok(mut x) = res {
            text = x.0;
            at += x.1;
            args.append(&mut x.2);
        } else {
            //println!("Exp End Error: {:?}", res);
            return res; 
        }
    }

    if !text.starts_with(";") { return Err(ParseErr{at: at + pos}); }
    at += 1;
    text = &text[1..];

    Ok((text, at, vec![Box::new(Item::Expr(op, args))]))
}

fn block_line_m<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    if let Ok(res) = block_blank_m(parse, input, pos) { return Ok(res); }
    if let Ok(res) = block_comment_m(parse, input, pos) { return Ok(res); } 
    if let Ok(res) = block_exp_m(parse, input, pos) { return Ok(res); }
    Err(ParseErr{at: pos})
}

fn block_main<'a, 'b>(parse: &'b Parse, input: &'a str, pos: usize) -> Result<(&'a str, usize, Vec<Box<Item>>), ParseErr> {
    let mut into = vec![];
    let mut text = input;
    let mut total = 0usize;
    loop {
        let res = block_line_m(parse, text, pos + total);
        if let Ok(mut x) = res {
            text = x.0;
            total += x.1;
            into.append(&mut x.2);
        } else { 
            //println!("Line Error: {:?}", res);
            break;
        }
    }
    Ok((text, total, into))
}