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
use std::collections::hash_map;
use std::rc::Rc;
use std::ops::Deref;
use std::sync::{Arc, atomic};

use core::fmt::{self, Formatter, Debug};

#[derive(Debug)]
enum DefPart {
    STR,
    ITEM,
    LIST,
}

#[derive(Debug, Clone)]
struct SecNext {
    num: Arc<atomic::AtomicUsize>
}

impl SecNext {
    pub fn new(first: usize) -> SecNext {
        SecNext {
            num: Arc::new(atomic::AtomicUsize::new(first))
        }
    }

    pub fn next(&self) -> usize {
        self.num.fetch_add(1, atomic::Ordering::Relaxed)
    }
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

    pub fn get_static(&self, block: usize, id: &String) -> Option<&StaticValue> {
        let mut at = block;
        loop {
            let b = &self.blocks[at];
            let v = b.statics.get(id);
            if v.is_some() { return v; }
            else if b.parent.is_some() { at = b.parent.unwrap(); }
            else { break; }
        }
        None
    }
}

struct BlockDat {
    pub id: String,
    pub index: usize,
    pub parent: Option<usize>,
    pub statics: HashMap<String, StaticValue>,
    pub dyns: HashMap<String, DynValue>,
    pub next_var: SecNext,
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
            dyns: HashMap::new(),
            next_var: SecNext::new(0),
            into_super: false,
            block: None,
        }
    }

    pub fn next_var(&self) -> String {
        format!("_v{}", self.next_var.next())
    }

    pub fn var_gen(&self) -> Box<Fn() -> String> {
        let gen = self.next_var.clone();
        Box::new(move || format!("_v{}", gen.next()))
    }
}

#[derive(Debug)]
enum DynValue {
    Val (Value),
    Capture {dat: CaptureDat},
    IntoList {vec_name: String},
    IntoOnce {let_name: String},
}

impl DynValue {
    pub fn deref(&self, vars: &Fn() -> String, code: &mut Vec<ast::Stmt>) -> Result<Value, String> {
        match self {
            &DynValue::Val(ref val) => Ok(val.clone()),
            &DynValue::Capture{dat: ref cap} => {
                let name = vars();
                code.push(StmtBuilder::new().let_()
                    .mut_id(&name)
                    .build_expr(cap.eval_ref()));
                Ok(Value::Str {var_name: name})
            },
            _ => Err(format!("{:?} is not a matchable value", self)),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Static (StaticValue),
    Str {var_name: String},
}

impl Value {
    pub fn gen_match(&self, dat: &mut BlockDat) -> Vec<ast::Stmt> {
        match self {
            &Value::Static(ref stat) => stat.gen_match(dat),
            &Value::Str{ref var_name} => vec![StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_match_str", vec![
                &|e| e.id("_pos"),
                &|e| e.id("_text"),
                &|e| e.id("TODO_out"),
                &|e| e.id(var_name),
            ]))]
        }
    }
}

#[derive(Debug, Clone)]
enum StaticValue {
    Regex {id: String},
    Str {value: String},
    Block {index: usize},
}

impl StaticValue {
    pub fn into_val(self) -> Value {
        Value::Static(self)
    }

    pub fn gen_match(&self, dat: &mut BlockDat) -> Vec<ast::Stmt> {
        match self {
            &StaticValue::Str{ref value} => {
                vec![StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_match_str", vec![
                    &|e| e.id("_pos"),
                    &|e| e.id("_text"),
                    &|e| e.id("TODO_out"),
                    &|e| e.str(&value[..]),
                ]))]
            },
            &StaticValue::Regex{ref id} => {
                vec![StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_match_regex", vec![
                    &|e| e.id("_pos"),
                    &|e| e.id("_text"),
                    &|e| e.id("TODO_out"),
                    &|e| e.field(id).id("_parse"),
                ]))]
            },
            &StaticValue::Block{ref index} => {
                vec![] // TODO
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CaptureDat {
    pub acc_name: Option<String>,
    pub start_name: String,
    pub is_ended: bool,
}

impl CaptureDat {
    pub fn start(vars: &Fn() -> String, code: &mut Vec<ast::Stmt>) -> CaptureDat {
        let dat = CaptureDat {
            acc_name: None,
            start_name: vars(),
            is_ended: false,
        };

        code.push(StmtBuilder::new().let_()
            .mut_id(&dat.start_name)
            .ty().usize()
            .build());

        dat
    }

    pub fn gen_start(&mut self) -> Vec<ast::Stmt> {
        self.is_ended = false;

        vec![StmtBuilder::new().expr().assign()
            .id(&self.start_name)
            .id("_pos")]
    }

    pub fn gen_end(&mut self, vars: &Fn() -> String) -> Vec<ast::Stmt> {
        let push = self.acc_name.is_some();

        if !push { self.acc_name = Some(vars()); }
        let name = self.acc_name.as_ref().unwrap();

        self.is_ended = true;

        if push {
            vec![StmtBuilder::new().expr().method_call("push_str")
                .id(name)
                .arg().ref_().index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_pos")
                .build()
            ]

        } else {
            vec![StmtBuilder::new().let_()
                .mut_id(name)
                .expr().call()
                    .path()
                        .global()
                        .ids(&["std", "string", "String", "from"])
                        .build()
                    .arg().ref_().index()
                        .id("_text")
                        .range()
                            .from().id(&self.start_name)
                            .to().id("_pos")
                    .build()
            ]
        }
    }

    pub fn eval_ref(&self) -> P<ast::Expr> {
        let expr = ExprBuilder::new();
        match (self.is_ended, self.acc_name.is_some()) {
            (false, true) => expr.ref_().index()
                .paren().add()
                    .method_call("clone")
                        .id(self.acc_name.as_ref().unwrap())
                        .build()
                    .ref_().index()
                        .id("_text")
                        .range()
                            .from().id(&self.start_name)
                            .to().id("_pos")
                .range().build(),
            (true, true) => expr.ref_().index()
                .id(self.acc_name.as_ref().unwrap())
                .range().build(),
            (false, false) => expr.ref_().index()
                .id("_text")
                .range()
                    .from().id(&self.start_name)
                    .to().id("_pos"),
            _ => panic!("Illegal capture state")
        }
    }

    pub fn eval_own(&self) -> P<ast::Expr> {
        let expr = ExprBuilder::new();
        match (self.is_ended, self.acc_name.is_some()) {
            (false, true) => expr.add()
                .method_call("clone")
                    .id(self.acc_name.as_ref().unwrap())
                    .build()
                .index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_pos"),
            (true, true) => expr.method_call("clone")
                .id(self.acc_name.as_ref().unwrap())
                .build(),
            (false, false) => expr.call()
                .path()
                    .global()
                    .ids(&["std", "string", "String", "from"])
                    .build()
                .arg().index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_pos")
                .build(),
            (true, false) => panic!("Illegal capture state")
        }
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
            let err = format!("\"capture\" expr \"capture <name>\" has invalid args: {:?}", args);

            if let [box Item::Name(ref name)] = args[..] {
                let vars = block.var_gen();

                match block.dyns.entry(name.clone()) {
                    hash_map::Entry::Occupied(mut e) => {
                        {if let &mut DynValue::Capture{dat: ref mut cap} = e.get_mut() {
                            return cap.gen_start();
                        }}
                        panic!(format!("Can not use existing value {:?} for capture", e.get()));
                    },
                    hash_map::Entry::Vacant(mut e) => {
                        let mut out = Vec::new();
                        let mut cap = CaptureDat::start(vars.as_ref(), &mut out);
                        out.append(&mut cap.gen_start());
                        e.insert(DynValue::Capture{dat: cap});
                        return out;
                    }
                }
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
            let err = format!("\"stop\" expr \"stop <name>\" has invalid args: {:?}", args);

            if let [box Item::Name(ref name)] = args[..] {
                let vars = block.var_gen();
                match block.dyns.get_mut(name) {
                    Some(&mut DynValue::Capture{dat: ref mut cap}) => {
                        return cap.gen_end(vars.as_ref());
                    },
                    // TODO
                    Some(v) => panic!(format!("Can not stop value {:?}", v)),
                    None => panic!(format!("No local value named {:?}", name)),
                }
            } else {
                panic!(err); // TODO no panics
            }
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
    let mut out = Vec::new();

    if let Ok(val) = gen_value(dat, block, op, &mut out) {
        out.append(&mut val.gen_match(&mut dat.blocks[block]));
        out
    } else if let &Item::Name(ref name) = op {
        compile_name_expr(dat, block, name, args)
    } else {
        panic!(format!("{:?} is not a matchable value or operation", op)) // TODO no panic
    }
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

fn gen_value<'a>(dat: &'a mut CompileData, block: usize, from: &Item, prefix: &mut Vec<ast::Stmt>) -> Result<Value, String> {
    {
        let blockdat = &mut dat.blocks[block];
        let vars = blockdat.var_gen();

        match from {
            &Item::Name(ref id) => {
                if let Some(l) = blockdat.dyns.get(id) {
                    return l.deref(vars.as_ref(), prefix);
                }
            },
            _ => (),
        }
    }

    let stat = gen_static_value(dat, block, from);
    if stat.is_ok() { Ok(Value::Static(stat.unwrap())) }
    else { Err(format!("{:?} is not a static or local value", from)) }
}

fn gen_static_value(dat: &mut CompileData, block: usize, from: &Item) -> Result<StaticValue, String> {
    match from {
        // compile block and add to global functions
        &Item::Block(ref lines) => {
            let index = dat.blocks.len();
            dat.blocks.push(BlockDat::new(index, Some(block)));

            dat.blocks[index].block = Some(compile_from_iter(dat, index, &lines[..]));

            Ok(StaticValue::Block{ index: index })
        }
        // add to global regex list
        &Item::Regex(ref source) => {
            let id = format!("_regex_{}", dat.regexs.len());
            dat.regexs.push((id.clone(), source.clone()));

            Ok(StaticValue::Regex{ id: id })
        }
        // string literal
        &Item::StrLiteral(ref value) => Ok(StaticValue::Str{ value: value.clone() }),
        &Item::Name(ref id) => {
            if let Some(value) = dat.get_static(block, id) { Ok(value.clone()) }
            else { Err(format!("{} does not name a prior static value", id)) }
        },
        _ => Err(format!("{:?} is not a static value", from))
    }
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