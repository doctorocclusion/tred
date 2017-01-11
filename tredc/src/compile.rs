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

use std::collections::{hash_map, HashMap};
use std::rc::Rc;
use std::ops::{Deref, Range, FnMut};
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

    pub fn next_name(&self, prefix: &str) -> String {
        format!("{}{}", prefix, self.next())
    }
} 

#[derive(Debug)]
struct CompileData {
    pub defs: Vec<(String, Vec<DefPart>)>,
    pub regexs: HashMap<String, usize>,
    pub blocks: Vec<BlockDat>,
    pub vars: SecNext,
}

impl CompileData {
    pub fn new() -> CompileData {
        CompileData {
            defs: Vec::new(),
            regexs: HashMap::new(),
            blocks: Vec::new(),
            vars: SecNext::new(0),
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
    pub active_into: Vec<IntoRec>,
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
            active_into: vec![IntoRec::Export],
            block: None,
        }
    }

    fn do_gen_append(&self, vars: &SecNext) -> P<ast::Block> {
        let mut lines = BlockBuilder::new();
        let only = self.active_into.len() == 1;

        for i in &self.active_into {
            if let Some(s) = i.append_part(only) { lines = lines.with_stmt(s); }
        }

        lines.build()
    }

    pub fn gen_append(&self, expr: ExprBuilder, vars: &SecNext) -> P<ast::Expr> {
        expr.paren().closure().by_ref() 
            .fn_decl()
                .arg()
                    .id("_vec")
                    .ty().ref_().mut_().ty().path()
                        .global()
                        .id("std")
                        .id("vec")
                        .segment("Vec")
                            .ty().id("Token")
                            .build()
                        .build()
                .no_return()
            .expr().build_block(self.do_gen_append(vars))
    }
}

#[derive(Debug, Clone)]
enum IntoRec {
    List(String),
    Once(String),
    Export,
}

impl IntoRec {
    pub fn append_part(&self, only: bool) -> Option<ast::Stmt> {
        match *self {
            IntoRec::Export => {
                let build = StmtBuilder::new();
                if only {
                    Some(build.expr().method_call("append")
                        .id("_out")
                        .arg()
                            .id("_vec")
                        .build())

                } else {
                    Some(build.expr().method_call("extend")
                        .id("_out")
                        .arg()
                            .method_call("iter")
                                .id("_vec")
                                .build()
                        .build())

                }
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
enum DynValue {
    Val (Value),
    Capture {dat: CaptureDat},
    IntoVal (IntoRec)
}

impl DynValue {
    pub fn deref(&self, vars: &SecNext, code: &mut Vec<ast::Stmt>) -> Result<Value, String> {
        match self {
            &DynValue::Val(ref val) => Ok(val.clone()),
            &DynValue::Capture{dat: ref cap} => {
                let name = vars.next_name("_cap_");
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
    pub fn gen_match(&self, pre: &mut Vec<ast::Stmt>) -> P<ast::Expr> {
        match self {
            &Value::Static(ref stat) => stat.gen_match(pre),
            &Value::Str{ref var_name} => ExprBuilder::new().build_mac(gen_mac("_tredgen_match_str", &mut [
                &mut |e| e.id("_pos"),
                &mut |e| e.id("_text"),
                &mut |e| e.id(var_name),
            ]))
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

    pub fn gen_match(&self, pre: &mut Vec<ast::Stmt>) -> P<ast::Expr> {
        match self {
            &StaticValue::Str{ref value} => {
                ExprBuilder::new().build_mac(gen_mac("_tredgen_match_str", &mut [
                    &mut |e| e.id("_pos"),
                    &mut |e| e.id("_text"),
                    &mut |e| e.str(&value[..]),
                ]))
            },
            &StaticValue::Regex{ref id} => {
                ExprBuilder::new().build_mac(gen_mac("_tredgen_match_regex", &mut [
                    &mut |e| e.id("_pos"),
                    &mut |e| e.id("_text"),
                    &mut |e| e.id(id),
                ]))
            },
            &StaticValue::Block{ref index} => {
                ExprBuilder::new().unit()
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
    pub fn start(vars: &SecNext, code: &mut Vec<ast::Stmt>) -> CaptureDat {
        let dat = CaptureDat {
            acc_name: None,
            start_name: vars.next_name("_start_"),
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

    pub fn gen_end(&mut self, vars: &SecNext) -> Vec<ast::Stmt> {
        let push = self.acc_name.is_some();

        if !push { self.acc_name = Some(vars.next_name("_acc_")); }
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

struct GenMatchExprBuilder<'a> {
    pub val: &'a Value,
}

impl <'a> GenMatchExprBuilder<'a> {
    pub fn new(val: &'a Value) -> GenMatchExprBuilder<'a> {
        GenMatchExprBuilder {
            val: val
        }
    }
}

fn gen_expr_vals(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Item>>) -> Vec<ast::Stmt> {
    let err = format!("\"{}\" expr \"{} <value> [<value>...]\" has invalid args: {:?}", op, op, args);
    let mut out = Vec::new();

    if args.len() < 1 {
        panic!(err); // TODO no panics
    }

    let vals: Vec<Value> = args.iter()
        .map(|b| gen_value(dat, blocki, b.as_ref(), &mut out).expect(&err))
        .collect();

    let block = &dat.blocks[blocki];

    let mut macargs: Vec<P<ast::Expr>> = vec![
        ExprBuilder::new().id("_pos"),
        ExprBuilder::new().id("_text"),
        block.gen_append( ExprBuilder::new(), &dat.vars),
    ];
    macargs.extend(vals.into_iter().map(|v| v.gen_match(&mut out)));

    out.push(StmtBuilder::new().expr().build_mac(gen_mac_direct(mac,macargs)));
    out
}

fn gen_expr_val(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Item>>) -> Vec<ast::Stmt> {
    let err = format!("\"{}\" expr \"{} <value>\" has invalid args: {:?}", op, op, args);

    if let [box ref val] = args[..] {
        let mut out = Vec::new();
        let val = gen_value(dat, blocki, val, &mut out).expect(&err);
        let block = &dat.blocks[blocki];

        let stmt = StmtBuilder::new().expr().build_mac(gen_mac(mac, &mut [
            &mut |e| e.id("_pos"),
            &mut |e| e.id("_text"),
            &mut |e| block.gen_append(e, &dat.vars),
            &mut |e| e.build(val.gen_match(&mut out)),
        ]));
        out.push(stmt);

        out
    } else {
        panic!(err); // TODO no panics
    }
}

fn compile_name_expr(dat: &mut CompileData, blocki: usize, op: &String, args: &Vec<Box<Item>>) -> Vec<ast::Stmt> {
    let vars = dat.vars.clone();

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
            gen_expr_val("_tredgen_not", "not", dat, blocki, args)
        },
        "capture" => {
            let err = format!("\"capture\" expr \"capture <name>\" has invalid args: {:?}", args);
            let mut block = &mut dat.blocks[blocki];

            if let [box Item::Name(ref name)] = args[..] {

                match block.dyns.entry(name.clone()) {
                    hash_map::Entry::Occupied(mut e) => {
                        {if let &mut DynValue::Capture{dat: ref mut cap} = e.get_mut() {
                            return cap.gen_start();
                        }}
                        panic!(format!("Can not use existing value {:?} for capture", e.get()));
                    },
                    hash_map::Entry::Vacant(e) => {
                        let mut out = Vec::new();
                        let mut cap = CaptureDat::start(&vars, &mut out);
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
            let mut block = &mut dat.blocks[blocki];

            if let [box Item::Name(ref name)] = args[..] {
                match block.dyns.get_mut(name) {
                    Some(&mut DynValue::Capture{dat: ref mut cap}) => {
                        return cap.gen_end(&vars);
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
            gen_expr_vals("_tredgen_some", "some", dat, blocki, args)
        },
        "many" => {
            gen_expr_vals("_tredgen_many", "many", dat, blocki, args)
        },
        "all" => {
            gen_expr_vals("_tredgen_all", "all", dat, blocki, args)
        },
        "option" => {
            gen_expr_vals("_tredgen_option", "option", dat, blocki, args)
        },
        "or" => {
            gen_expr_vals("_tredgen_or", "option", dat, blocki, args)
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
        let stmt = StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_append", &mut [
            &mut |e| e.id("_pos"),
            &mut |e| e.id("_text"),
            &mut |e| dat.blocks[block].gen_append(e, &dat.vars),
            &mut |e| e.try().build(val.gen_match(&mut out)),
        ]));
        out.push(stmt);
        out
    } else if let &Item::Name(ref name) = op {
        compile_name_expr(dat, block, name, args)
    } else {
        panic!(format!("{:?} is not a matchable value or operation", op)) // TODO no panic
    }
}

fn gen_mac_direct(name: &str, exprs: Vec<P<ast::Expr>>) ->  ast::Mac {
    let mut mac = MacBuilder::new().path().id(name).build();
    for e in exprs {
        mac = mac.expr().build(e)
        .expr().id(",");

    }
    mac.build()
}

fn gen_mac(name: &str, exprs: &mut [&mut FnMut(ExprBuilder) -> P<ast::Expr>]) ->  ast::Mac {
    gen_mac_direct(name, exprs.iter_mut().map(|f| f(ExprBuilder::new())).collect())
}

fn gen_value<'a>(dat: &'a mut CompileData, block: usize, from: &Item, prefix: &mut Vec<ast::Stmt>) -> Result<Value, String> {
    let vars = dat.vars.clone();

    {
        let blockdat = &mut dat.blocks[block];

        match from {
            &Item::Name(ref id) => {
                if let Some(l) = blockdat.dyns.get(id) {
                    return l.deref(&vars, prefix);
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
            let index = dat.regexs.entry(source.clone()).or_insert(dat.vars.next());
            let id = format!("_regex_{}", index);

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
    code = code.stmt().let_()
        .mut_id("_pos")
        .expr().id("_i_pos");
    code = code.stmt().let_()
        .mut_id("_text")
        .expr().index().ref_()
            .id("_i_text")
            .range().build();
        code = code.stmt().let_()
        .mut_id("_out")
        .expr().call()
            .path()
                .global()
                .ids(&["std", "vec", "Vec", "new"])
                .build()
            .build();

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