use syntax::{self};
use aster::block::{BlockBuilder};

use tredlib::{ParseErr};
use tredlib::regex::{self};

use parse::{self, Parse, Item};

use unescape::unescape;

use std::collections::HashMap;

use core::fmt::{self, Formatter, Debug};

#[derive(Debug)]
enum DefPart {
    STR,
    ITEM,
    LIST,
}

#[derive(Debug)]
struct CompileData {
    pub defs: Vec<(String, Vec<DefPart>)>,
    pub regexs: Vec<(String, String)>,
    pub blocks: Vec<BlockDat>,
}

impl CompileData {
    pub fn new() -> CompileData {
        CompileData {
            defs: Vec::new(),
            regexs: Vec::new(),
            blocks: Vec::new(),
        }
    }
}

struct BlockDat {
    pub id: String,
    pub index: usize,
    pub parent: Option<usize>,
    pub statics: HashMap<String, Value>,
    pub code: BlockBuilder,
}

impl Debug for BlockDat {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("BlockDat {{ id: \"{}\", parent: {:?}, statics: {:?} }}", self.id, self.index, self.statics))
    }
}

impl BlockDat {
    pub fn new(index: usize, parent: Option<usize>) -> BlockDat {
        BlockDat {
            id: format!("_blockfn_{}", index),
            index: index,
            parent: parent,
            statics: HashMap::new(),
            code: BlockBuilder::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Regex {
        id: String,
    },
    StringLit {
        value: String,
    },
    Block {
        index: usize,
    },
    StatRef {
        val: Box<Value>
    }
}

impl Value {
    fn gen_match(&self, code: &mut BlockDat) {
        // TODO
    }
}

fn compile_name_expr(dat: &mut CompileData, block: usize, op: &String, args: &Vec<Box<Item>>) {
    let block = &mut dat.blocks[block];
    let code = &mut block.code;

    match &op[..] {
        // def expression
        "def" => {
            let err = format!("\"def\" expr \"def <name> [<str|item|list> ...]\" has invalid args: {:?}", args);
            // get the new defined tuple/enum's name (first arg)
            if let [box Item::Name(ref name), ..] = args[..] {
                let mut parts = Vec::new();
                for i in &args[1..] {
                    // for each type in the tuple (remaining args)
                    if let &box Item::Name(ref ty) = i {
                        parts.push(match &ty.to_lowercase()[..] {
                            "str" => DefPart::STR,
                            "item" => DefPart::ITEM,
                            "list" => DefPart::LIST,
                            _ => panic!(err)
                        });
                    } else {
                        panic!(err);
                    }
                }
                // add def
                dat.defs.push((name.clone(), parts));
            } else {
                panic!(err); // TODO no panics
            }
        },
        // stat expression (already handled)
        "stat" => (),
        // no other expressions
        _ => ()
    }

    // TODO
}
 
fn compile_expr(dat: &mut CompileData, block: usize, op: &Item, args: &Vec<Box<Item>>) {
    match op {
        &Item::Name(ref name) => {
            compile_name_expr(dat, block, name, args);
        },
        &Item::Block(_) | &Item::Regex(_) | &Item::StrLiteral(_) => {
            compile_value(dat, block, op).unwrap().gen_match(&mut dat.blocks[block]); // TODO no unwraps
        },
        _ => panic!(format!("{:?} is not a valid operation", op))
    }
}

fn compile_value(dat: &mut CompileData, block: usize, value: &Item) -> Result<Value, String> {
    match value {
        // compile block and add to global functions
        &Item::Block(ref lines) => {
            let index = dat.blocks.len();
            dat.blocks.push(BlockDat::new(index, Some(block)));

            compile_from_iter(dat, index, &lines[..]);

            Ok(Value::Block{ index: index })
        }
        // add to global regex list
        &Item::Regex(ref source) => {
            let id = format!("_regex_{}", dat.regexs.len());
            dat.regexs.push((id.clone(), source.clone()));

            Ok(Value::Regex{ id: id })
        }
        // string literal
        &Item::StrLiteral(ref value) => Ok(Value::StringLit{ value: value.clone() }),
        _ => Err(format!("{:?} is not a value", value))
    }
}

fn compile_from_iter(dat: &mut CompileData, block: usize, toks: &[Box<Item>]) {
    // find and compile statics
    for i in toks {
        match i {
            &box Item::Expr(box Item::Name(ref stat), ref args) if stat == "stat" => {
                // has form "stat name value;"

                // check name and value
                if let [box Item::Name(ref name), box ref value] = args[..] {
                    let v = compile_value(dat, block, value).unwrap();
                    dat.blocks[block].statics.insert(name.clone(), v);  // TODO no unwraps
                } else {
                    panic!(format!("\"stat\" expr \"stat <name> <value>\" has invalid args: {:?}", args))
                }
            },
            _ => ()
        }
    }

    // actually compile
    for i in toks {
        match i {
            &box Item::Expr(ref op, ref args) => compile_expr(dat, block, op.as_ref(), args),
            &box Item::Comment(ref text) => (),
            _ => panic!(format!("{:?} is not a valid program line", i))
        }
    }
}

pub fn compile(toks: &[Box<Item>]) {
    let mut dat = CompileData::new();
    dat.blocks.push(BlockDat::new(0, None));
    compile_from_iter(&mut dat, 0, toks);

    println!("{:?}", dat);

    // TODO enums, funcs, and final output
}