use syntax::{self, ast};
use syntax::parse::token::{Token};
use syntax::ptr::P;

use aster::block::BlockBuilder;
use aster::expr::ExprBuilder;
use aster::stmt::StmtBuilder;
use aster::mac::MacBuilder;

use tredlib::{ParseErr};
use tredlib::regex::{self};

use parse::{self, Parse, Item};

use unescape::unescape;

use std::collections::HashMap;
use std::rc::Rc;

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
    pub locals: HashMap<String, Rc<LocalVal>>,
    pub next_var: u32,
    pub into_super: bool,
    pub block: Option<ast::Block>,
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
            locals: HashMap::new(),
            next_var: 0,
            into_super: false,
            block: None,
        }
    }

    pub fn next_var(&mut self) -> String {
        let out = format!("_v{}", self.next_var);
        self.next_var += 1;
        return out;
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
    Local ( Rc<LocalVal> ),
}

impl Value {
    fn gen_match(&self, dat: &mut BlockDat) -> Vec<ast::Stmt> {
        match self {
            &Value::StringLit{ref value} => {
                vec![StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_match_str", vec![
                    &|e| e.id("_pos"),
                    &|e| e.id("_text"),
                    &|e| e.id("TODO_out"),
                    &|e| e.str(&value[..]),
                ]))]
            },
            &Value::Regex{ref id} => {
                vec![StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_match_regex", vec![
                    &|e| e.id("_pos"),
                    &|e| e.id("_text"),
                    &|e| e.id("TODO_out"),
                    &|e| e.field(id).id("_parse"),
                ]))]
            },
            _ => vec![], // TODO
        }
        // TODO
    }
}

#[derive(Debug, Clone)]
struct CaptureDat {
    pub acc_name: Option<String>,
    pub start_name: String,
    pub is_ended: bool,
}

impl CaptureDat {
    pub fn start(dat: &mut BlockDat) -> (Vec<ast::Stmt>, CaptureDat) {
        let dat = CaptureDat {
            acc_name: None,
            start_name: dat.next_var(),
            is_ended: false,
        };
        let init = vec![
            StmtBuilder::new().let_()
                .mut_id(&dat.start_name)
                .ty().usize()
                .build()];
        (init, dat)
    }

    pub fn gen_start(&mut self, dat: &mut BlockDat) -> Vec<ast::Stmt> {
        self.is_ended = false;

        vec![StmtBuilder::new().expr().assign()
            .id(&self.start_name)
            .id("_pos")]
    }

    pub fn gen_end(&mut self, dat: &mut BlockDat) -> Vec<ast::Stmt> {
        let push = self.acc_name.is_some();

        if !push { self.acc_name = Some(dat.next_var()); }
        let name = self.acc_name.as_ref().unwrap();

        self.is_ended = true;

        if push {
            vec![StmtBuilder::new().expr().method_call("push_str")
                .id(name)
                .arg().index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_pos")
                .build()
            ]

        } else {
            vec![StmtBuilder::new().let_()
                .mut_id(name)
                .ty().usize()
                .expr().call()
                    .path()
                        .global()
                        .ids(&["std", "string", "String", "from"])
                        .build()
                    .arg().index()
                        .id("_text")
                        .range()
                            .from().id(&self.start_name)
                            .to().id("_pos")
                    .build()
            ]
        }
    }
}

#[derive(Debug, Clone)]
enum LocalVal {
    Capture ( CaptureDat ),
    IntoList {
        vec_name: String,
    },
    IntoOnce {
        let_name: String,
    }
}

fn compile_name_expr(dat: &mut CompileData, block: usize, op: &String, args: &Vec<Box<Item>>) -> Vec<ast::Stmt> {
    let mut block = &mut dat.blocks[block];

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

            Vec::new()
        },
        // stat expression (already handled)
        "stat" => Vec::new(),
        "not" => {
            Vec::new()
        },
        "capture" => {
            let err = format!("\"capture\" expr \"def <name>\" has invalid args: {:?}", args);

            if let [box Item::Name(ref name)] = args[..] {
                vec![] // TODO
            } else {
                panic!(err); // TODO no panics
            }
        },
        "into" => {
            Vec::new()
        },
        "into_once" => {
            Vec::new()
        },
        "stop" => {
            Vec::new()
        },
        "some" => {
            Vec::new()
        },
        "many" => {
            Vec::new()
        },
        "all" => {
            Vec::new()
        },
        "option" => {
            Vec::new()
        },
        "or" => {
            Vec::new()
        },
        "export" => {
            Vec::new()
        },
        // no other expressions
        _ => Vec::new()
    }
}
 
fn compile_expr(dat: &mut CompileData, block: usize, op: &Item, args: &Vec<Box<Item>>) -> Vec<ast::Stmt> {
    if let Ok(val) = gen_value(dat, block, op) {
        val.gen_match(&mut dat.blocks[block])
    } else if let &Item::Name(ref name) = op {
        compile_name_expr(dat, block, name, args)
    } else {
        panic!(format!("{:?} is not a valid static value, local value, or operation", op))
    }
}

fn get_static<'a>(dat: &'a CompileData, lowest_block: usize, id: &String) -> Option<&'a Value> {
    let mut bl = lowest_block;
    loop {
        let block = &dat.blocks[bl];
        if let Some(val) = block.statics.get(id) { return Some(val) }
        else {
            if let Some(parent) = block.parent { bl = parent; }
            else { break; }
        }
    }
    None
}

fn gen_mac(name: &str, exprs: Vec<&Fn(ExprBuilder) -> P<ast::Expr>>) ->  ast::Mac {
    let mut mac = MacBuilder::new().path().id(name).build();
    for e in exprs {
        mac = 
            mac.expr().build(e(ExprBuilder::new()))
            .expr().id(",");

    }
    mac.build()
}

fn gen_value(dat: &mut CompileData, block: usize, value: &Item) -> Result<Value, String> {
    {
        let blockdat = &mut dat.blocks[block];

        match value {
            &Item::Name(ref id) => {
                if let Some(l) = blockdat.locals.get(id) {
                    return Ok(Value::Local(l.clone()));
                }
            },
            // TODO
            _ => ()
        }
    }

    {
        let stat = gen_static_value(dat, block, value);
        if stat.is_ok() { return stat; }
    }

    Err(format!("{:?} is not a static or local value", value))
}

fn gen_static_value(dat: &mut CompileData, block: usize, value: &Item) -> Result<Value, String> {
    match value {
        // compile block and add to global functions
        &Item::Block(ref lines) => {
            let index = dat.blocks.len();
            dat.blocks.push(BlockDat::new(index, Some(block)));

            dat.blocks[index].block = Some(compile_from_iter(dat, index, &lines[..]));

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
        &Item::Name(ref id) => {
            let ent = get_static(dat, block, id);
            if let Some(val) = ent { Ok(val.clone()) }
            else { Err(format!("{} does not name a prior static value", id)) }
        },
        _ => Err(format!("{:?} is not a static value", value))
    }
}

fn gen_capture_str(capture: &CaptureDat) -> ast::Expr {
    let expr = ExprBuilder::new();
    match (capture.is_ended, capture.acc_name.is_some()) {
        (false, true) => expr.add()
            .method_call("clone")
                .id(capture.acc_name.as_ref().unwrap())
                .build()
            .index()
                .id("_text")
                .range()
                    .from().id(&capture.start_name)
                    .to().id("_pos"),
        (true, true) => expr.id(capture.acc_name.as_ref().unwrap()),
        (false, false) => expr.index()
            .id("_text")
            .range()
                .from().id(&capture.start_name)
                .to().id("_pos"),
        (true, false) => panic!("Illegal capture state")
    }.unwrap()
}

fn compile_from_iter(dat: &mut CompileData, block: usize, toks: &[Box<Item>]) -> ast::Block {
    // find and compile statics
    for i in toks {
        match i {
            &box Item::Expr(box Item::Name(ref stat), ref args) if stat == "stat" => {
                // has form "stat name value;"

                // check name and value
                if let [box Item::Name(ref name), box ref value] = args[..] {
                    let v = gen_static_value(dat, block, value).unwrap();
                    dat.blocks[block].statics.insert(name.clone(), v);  // TODO no unwraps
                } else {
                    panic!(format!("\"stat\" expr \"stat <name> <value>\" has invalid args: {:?}", args))
                }
            },
            _ => ()
        }
    }

    let mut code = BlockBuilder::new();

    // actually compile
    for t in toks {
        match t {
            &box Item::Expr(ref op, ref args) => code = code.with_stmts(
                compile_expr(dat, block, op.as_ref(), args).into_iter()),
            &box Item::Comment(_) => (),
            _ => panic!(format!("{:?} is not a valid program line", t)),
        }
    }

    code.build().unwrap()
}

pub fn compile(toks: &[Box<Item>]) {
    let mut dat = CompileData::new();
    dat.blocks.push(BlockDat::new(0, None));
    dat.blocks[0].block = Some(compile_from_iter(&mut dat, 0, toks));

    println!("{:?}", dat);

    for b in dat.blocks {
        println!("\n========\n{}", syntax::print::pprust::block_to_string(&b.block.unwrap()));
    }
    // TODO enums, funcs, and final output
}