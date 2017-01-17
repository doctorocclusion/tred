use syntax::{self, ast};
use syntax::ptr::P;

use aster::block::BlockBuilder;
use aster::expr::ExprBuilder;
use aster::stmt::StmtBuilder;
use aster::mac::MacBuilder;
use aster::item::ItemBuilder;
use aster::ty::TyBuilder;

use tredlib::{ParseErr};
use tredlib::regex::{self};

use parse::{Token};

use unescape::unescape;

use std::collections::{hash_map, HashMap};
use std::rc::Rc;
use std::ops::{Deref, Range, FnMut};
use std::sync::{Arc, atomic};

use core::fmt::{self, Formatter, Debug};

#[derive(Debug, Clone, Copy)]
enum DefPart {
    STR,
    ITEM,
    LIST,
}

impl DefPart {
    pub fn ty(&self) -> P<ast::Ty> {
        let ty = TyBuilder::new();
        match self {
            &DefPart::STR => ty.path().global().ids(&["std", "string", "String"]).build(),
            &DefPart::ITEM => ty.option().box_().id("Token"),
            &DefPart::LIST => ty.path()
                .global()
                .id("std")
                .id("vec")
                .segment("Vec")
                    .ty().box_().id("Token")
                    .build()
                .build()
        }
    }
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

#[derive(Debug)]
struct BlockDat {
    pub id: String,
    pub index: usize,
    pub parent: Option<usize>,
    pub statics: HashMap<String, StaticValue>,
    pub dyns: HashMap<String, DynValue>,
    pub active_into: Vec<IntoRec>,
    pub block: Option<P<ast::Block>>,
}

impl BlockDat {
    pub fn new(index: usize, parent: Option<usize>) -> BlockDat {
        BlockDat {
            id: format!("_blockfn_{}", index),
            index: index,
            parent: parent,
            statics: HashMap::new(),
            dyns: HashMap::new(),
            active_into: Vec::new(),
            block: None,
        }
    }

    fn do_gen_append(&self, vars: &SecNext, vec: &str) -> P<ast::Block> {
        let mut lines = BlockBuilder::new();
        let count = self.active_into.len();

        for i in 0..count {
            if let Some(s) = self.active_into[i].append_part(vec, i == (count - 1)) { lines = lines.with_stmt(s); }
        }

        lines.build()
    }

    pub fn gen_append(&self, expr: ExprBuilder, vars: &SecNext, vec: &str, ) -> P<ast::Expr> {
        expr.build_block(self.do_gen_append(vars, vec))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntoRec {
    List(String),
    Once(String, bool),
}

impl IntoRec {
    pub fn as_vec(&self, leave: bool) -> P<ast::Expr> {
        match (self, leave) {
            (&IntoRec::Once(ref var, false), _) => {
                ExprBuilder::new()
                .method_call("collect")
                .method_call("cloned")
                .method_call("iter")
                    .id(var)
                .build()
                .build()
                .build()
            },
            (&IntoRec::Once(ref var, true), true) => {
                 ExprBuilder::new()
                .method_call("clone")
                    .id(var)
                .build()
            },
            (&IntoRec::List(ref var), true) => {
                ExprBuilder::new()
                .method_call("clone")
                    .id(var)
                .build()
            },
            (&IntoRec::Once(ref var, true), false) => {
                ExprBuilder::new().id(var)

            },
            (&IntoRec::List(ref var), false) => {
                ExprBuilder::new().id(var)
            },
        }
    }

    pub fn as_opt(&self, leave: bool) -> P<ast::Expr> {
        match (self, leave) {
            (&IntoRec::Once(ref var, false), true) => {
                ExprBuilder::new()
                .method_call("clone")
                    .id(var)
                .build()
            },
            (&IntoRec::Once(ref var, true), true) => {
                ExprBuilder::new()
                .method_call("next")
                .method_call("cloned")
                .method_call("iter")
                    .id(var)
                .build()
                .build()
                .build()
            },
            (&IntoRec::List(ref var), true) => {
                ExprBuilder::new()
                .method_call("next")
                .method_call("cloned")
                .method_call("iter")
                    .id(var)
                .build()
                .build()
                .build()
            },
            (&IntoRec::Once(ref var, false), false) => {
                ExprBuilder::new().id(var)

            },
            (&IntoRec::Once(ref var, true), false) => {
                ExprBuilder::new()
                .method_call("next")
                .method_call("into_iter")
                    .id(var)
                .build()
                .build()

            },
            (&IntoRec::List(ref var), false) => {
                ExprBuilder::new()
                .method_call("next")
                .method_call("into_iter")
                    .id(var)
                .build()
                .build()
            },
        }
    }

    pub fn append_part(&self, vec: &str, only: bool) -> Option<ast::Stmt> {
        match *self {
            IntoRec::Once(ref var, list) => {
                let build = StmtBuilder::new();
                match (list, only) {
                    (true, true) => Some(build.expr().method_call("append")
                        .id(var)
                        .arg().id(vec)
                        .build()),
                    (true, false) => Some(build.expr().method_call("extend")
                        .id(var)
                        .arg()
                            .method_call("cloned")
                            .method_call("iter")
                            .id(vec)
                            .build()
                            .build()
                        .build()),
                    (false, true) => Some(build.expr().assign()
                        .id(var)
                        .method_call("or")
                            .method_call("pop")
                                .id(vec)
                                .build()
                            .arg().id(var)
                            .build()),
                    (false, false) => Some(build.expr().assign()
                        .id(var)
                        .method_call("or")
                            .method_call("cloned")
                                .method_call("last")
                                    .id(vec)
                                    .build()
                                .build()
                            .arg().id(var)
                            .build())
                }
            },
            IntoRec::List(ref var) => {
                let build = StmtBuilder::new();
                if only {
                    Some(build.expr().method_call("append")
                        .id(var)
                        .arg()
                            .id(vec)
                        .build())

                } else {
                    Some(build.expr().method_call("extend")
                        .id(var)
                        .arg()
                            .method_call("cloned")
                            .method_call("iter")
                            .id(vec)
                            .build()
                            .build()
                        .build())

                }
            }
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
                &mut |e| e.id("_at"),
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
                    &mut |e| e.id("_at"),
                    &mut |e| e.id("_text"),
                    &mut |e| e.str(&unescape(value).unwrap()[..]),
                ]))
            },
            &StaticValue::Regex{ref id} => {
                ExprBuilder::new().build_mac(gen_mac("_tredgen_match_regex", &mut [
                    &mut |e| e.id("_at"),
                    &mut |e| e.id("_text"),
                    &mut |e| e.id(id),
                ]))
            },
            &StaticValue::Block{ref index} => {
                ExprBuilder::new().call()
                    .id(format!("_blockfn_{}", index))
                    .arg().id("_at")
                    .arg().id("_text")
                    .build()
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
            .id("_at")]
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
                        .to().id("_at")
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
                            .to().id("_at")
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
                            .to().id("_at")
                .range().build(),
            (true, true) => expr.ref_().index()
                .id(self.acc_name.as_ref().unwrap())
                .range().build(),
            (false, false) => expr.ref_().index()
                .id("_text")
                .range()
                    .from().id(&self.start_name)
                    .to().id("_at"),
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
                .ref_().index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_at"),
            (true, true) => expr.method_call("clone")
                .id(self.acc_name.as_ref().unwrap())
                .build(),
            (false, false) => expr.call()
                .path()
                    .global()
                    .ids(&["std", "string", "String", "from"])
                    .build()
                .arg().ref_().index()
                    .id("_text")
                    .range()
                        .from().id(&self.start_name)
                        .to().id("_at")
                .build(),
            (true, false) => panic!("Illegal capture state")
        }
    }
}

fn gen_expr_vals(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Token>>) -> Vec<ast::Stmt> {
    let err = format!("\"{}\" expr \"{} <value> [<value>...]\" has invalid args: {:?}", op, op, args);
    let mut out = Vec::new();

    if args.len() < 1 {
        panic!(err); // TODO no panics
    }

    let vals: Vec<Value> = args.iter()
        .map(|b| gen_value(dat, blocki, b.as_ref(), &mut out).expect(&err))
        .collect();

    let block = &dat.blocks[blocki];
    let res_vec = dat.vars.next_name("_resvec_");

    let mut macargs: Vec<P<ast::Expr>> = vec![
        ExprBuilder::new().id("_at"),
        ExprBuilder::new().id("_text"),
        ExprBuilder::new().id(&res_vec),

        block.gen_append(ExprBuilder::new(), &dat.vars, &res_vec),
    ];
    macargs.extend(vals.into_iter().map(|v| v.gen_match(&mut out)));

    out.push(StmtBuilder::new().expr().build_mac(gen_mac_direct(mac,macargs)));
    out
}

fn gen_expr_val(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Token>>) -> Vec<ast::Stmt> {
    let err = format!("\"{}\" expr \"{} <value>\" has invalid args: {:?}", op, op, args);

    if let [box ref val] = args[..] {
        let mut out = Vec::new();
        let val = gen_value(dat, blocki, val, &mut out).expect(&err);
        let block = &dat.blocks[blocki];
        let res_vec = dat.vars.next_name("_resvec_");

        let stmt = StmtBuilder::new().expr().build_mac(gen_mac(mac, &mut [
            &mut |e| e.id("_at"),
            &mut |e| e.id("_text"),
            &mut |e| e.id(&res_vec),
            &mut |e| block.gen_append(e, &dat.vars, &res_vec),
            &mut |e| e.build(val.gen_match(&mut out)),
        ]));
        out.push(stmt);

        out
    } else {
        panic!(err);
    }
}

fn gen_token_output(dat: &CompileData, block: usize, def: DefPart, item: &Token) -> Result<P<ast::Expr>, String>  {
    match (def, item) {
        (DefPart::ITEM, &Token::Tuple(Some(box Token::Name(ref name)), ref parts)) => {
            let part_defs = dat.defs.iter().find(|v| v.0 == &name[..]);
            if part_defs.is_none() { return Err(format!("{} is an unknown definition", name)); }
            let part_defs = &part_defs.unwrap().1;

            if part_defs.len() != parts.len() { return Err(format!("{} has {} members, only {} were supplied", name, part_defs.len(), parts.len())); }

            if !part_defs.is_empty() {
                let mut eb = ExprBuilder::new().some().box_().call()
                    .path().ids(&["Token", name]).build();

                for i in 0..part_defs.len() {
                    match gen_token_output(dat, block, part_defs[i], parts[i].as_ref()) {
                        Ok(e) => eb = eb.with_arg(e),
                        e @ _ => return e,
                    }
                }

                Ok(eb.build())
            } else {
                Ok(ExprBuilder::new().some().box_()
                    .path().ids(&["Token", name]).build())
            }
        },
        (DefPart::ITEM, &Token::Name(ref name)) => {
            match dat.blocks[block].dyns.get(name) {
                Some(&DynValue::IntoVal(ref into)) => Ok(into.as_opt(true)),
                Some(v @ _) => Err(format!("{} ({:?}) is dynamic but has not received into or into_once", name, v)),
                None => Err(format!("{} does not name a value", name)),
            }
        },
        (DefPart::LIST, &Token::Name(ref name)) => {
            match dat.blocks[block].dyns.get(name) {
                Some(&DynValue::IntoVal(ref into)) => Ok(into.as_vec(true)),
                Some(v @ _) => Err(format!("{} ({:?}) is dynamic but has not received into or into_once", name, v)),
                None => Err(format!("{} does not name a value", name)),
            }
        },
        (DefPart::STR, &Token::Name(ref name)) => {
            match dat.blocks[block].dyns.get(name) {
                Some(&DynValue::Capture {dat: ref dat}) => Ok(dat.eval_own()),
                Some(v @ _) => Err(format!("{} ({:?}) is dynamic but not a capture", name, v)),
                None => match dat.blocks[block].statics.get(name) {
                    Some(&StaticValue::Str {value: ref val}) => Ok(ExprBuilder::new().str(&unescape(val).unwrap()[..])),
                    Some(v @ _) => Err(format!("{} ({:?}) is static but not a string", name, v)),
                    None => Err(format!("{} does not name a value", name)),
                }
            }


        },
        (DefPart::STR, &Token::StrLiteral(ref val)) => {
            Ok(ExprBuilder::new().str(&unescape(val).unwrap()[..]))
        },
        _ => Err(format!("{:?} is not a valid as a {:?} member", item, def))
    }
}

fn compile_name_expr(dat: &mut CompileData, blocki: usize, op: &String, args: &Vec<Box<Token>>) -> Vec<ast::Stmt> {
    let vars = dat.vars.clone();

    match &op[..] {
        // def expression (already handled)
        "def" => Vec::new(),
        // stat expression (already handled)
        "stat" => Vec::new(),
        "not" => {
            gen_expr_val("_tredgen_not", "not", dat, blocki, args)
        },
        "capture" => {
            let err = format!("\"capture\" expr \"capture <name>\" has invalid args: {:?}", args);
            let mut block = &mut dat.blocks[blocki];

            if let [box Token::Name(ref name)] = args[..] {

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
                panic!(err);
            }
        },
        "into" => {
            let err = format!("\"into\" expr \"into <name>\" has invalid args: {:?}", args);
            let mut block = &mut dat.blocks[blocki];

            let name;
            if let [box Token::Name(ref n)] = args[..] {
                match block.dyns.get(n) {
                    Some(&DynValue::IntoVal(ref rec @ IntoRec::List(_))) => {
                        if block.active_into.contains(&rec) {
                            panic!(format!("Already appending into \"{}\"", n));
                        }
                        block.active_into.push(rec.clone());
                        return Vec::new();
                    },
                    Some(v @ _) => panic!(format!("Can not append into \"{}\", already has value: {:?}", n, v)),
                    None => name = n,
                }
            } else {
                panic!(err);
            }

            let exporting = name == "export";
            let id = if exporting { String::from("_out") } else { vars.next_name("_intolist_") };
            let rec = IntoRec::List(id.clone());
            block.active_into.push(rec.clone());
            block.dyns.insert(name.clone(), DynValue::IntoVal(rec));
            if exporting { Vec::new() } else {
                vec![StmtBuilder::new().let_()
                .mut_id(&id)
                .expr().call()
                    .path()
                        .global()
                        .ids(&["std", "vec", "Vec", "new"])
                        .build()
                    .build()]
            }
        },
        "into_once" => {
            let err = format!("\"into_once\" expr \"into_once <name>\" has invalid args: {:?}", args);
            let mut block = &mut dat.blocks[blocki];

            let name;
            if let [box Token::Name(ref n)] = args[..] {
                match block.dyns.get(n) {
                    Some(&DynValue::IntoVal(ref rec @ IntoRec::Once(_, _))) => {
                        if block.active_into.contains(&rec) {
                            panic!(format!("Already assigning into \"{}\"", n));
                        }
                        block.active_into.push(rec.clone());
                        return Vec::new();
                    },
                    Some(v @ _) => panic!(format!("Can not assign into \"{}\", already has value: {:?}", n, v)),
                    None => name = n,
                }
            } else {
                panic!(err);
            }

            let exporting = name == "export";
            let id = if exporting { String::from("_out") } else { vars.next_name("_intoonce_") };
            let rec = IntoRec::Once(id.clone(), exporting);
            block.active_into.push(rec.clone());
            block.dyns.insert(name.clone(), DynValue::IntoVal(rec));
            if exporting { Vec::new() } else {
                vec![StmtBuilder::new().let_()
                    .mut_id(&id)
                    .expr().none()]
            }
        },
        "stop" => {
            let err = format!("\"stop\" expr \"stop <name>\" has invalid args: {:?}", args);
            let mut block = &mut dat.blocks[blocki];

            if let [box Token::Name(ref name)] = args[..] {
                let remove;

                match block.dyns.get_mut(name) {
                    Some(&mut DynValue::Capture{dat: ref mut cap}) => return cap.gen_end(&vars),
                    Some(&mut DynValue::IntoVal(ref v)) => remove = v,
                    // TODO
                    Some(v) => panic!(format!("Can not stop value {:?}", v)),
                    None => panic!(format!("No local value named {:?}", name)),
                }

                if let Some((i, _)) = block.active_into.iter().enumerate().find(|v| v.1 == remove) {
                    block.active_into.swap_remove(i);
                } else {
                    panic!(format!("Can not stop nonexistent value {}", name))
                }

                Vec::new()
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
            let err = format!("\"export\" expr \"export <enum>(<data>...)\" has invalid args: {:?}", args);
            if let [box ref item] = args[..] {
                vec![StmtBuilder::new().expr().method_call("extend")
                    .id("_out")
                    .with_arg(gen_token_output(dat, blocki, DefPart::ITEM, item).expect(&err))
                    .build()
                ]
            } else {
                panic!(err);
            }  
        },
        // no other expressions
        op @ _ => panic!(format!("Unknown operation: {}", op)),
    }
}
 
fn compile_expr(dat: &mut CompileData, block: usize, op: &Token, args: &Vec<Box<Token>>) -> Vec<ast::Stmt> {
    let mut out = Vec::new();

    if let Ok(val) = gen_value(dat, block, op, &mut out) {
        let res_vec = dat.vars.next_name("_resvec_");

        let stmt = StmtBuilder::new().expr().build_mac(gen_mac("_tredgen_append", &mut [
            &mut |e| e.id("_at"),
            &mut |e| e.id(&res_vec),
            &mut |e| dat.blocks[block].gen_append(e, &dat.vars, &res_vec),
            &mut |e| e.try().build(val.gen_match(&mut out)),
        ]));
        out.push(stmt);
        out
    } else if let &Token::Name(ref name) = op {
        compile_name_expr(dat, block, name, args)
    } else {
        panic!(format!("{:?} is not a matchable value or operation", op)) // TODO no panic
    }
}

fn gen_mac_direct(name: &str, exprs: Vec<P<ast::Expr>>) ->  ast::Mac {
    let mut mac = MacBuilder::new().path().id(name).build();
    let mut first = true;
    for e in exprs {
        if !first { mac = mac.expr().id(","); }
        else { first = false; }
        mac = mac.expr().build(e);

    }
    mac.build()
}

fn gen_mac(name: &str, exprs: &mut [&mut FnMut(ExprBuilder) -> P<ast::Expr>]) ->  ast::Mac {
    gen_mac_direct(name, exprs.iter_mut().map(|f| f(ExprBuilder::new())).collect())
}

fn gen_value<'a>(dat: &'a mut CompileData, block: usize, from: &Token, prefix: &mut Vec<ast::Stmt>) -> Result<Value, String> {
    let vars = dat.vars.clone();

    {
        let blockdat = &mut dat.blocks[block];

        match from {
            &Token::Name(ref id) => {
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

fn gen_static_value(dat: &mut CompileData, block: usize, from: &Token) -> Result<StaticValue, String> {
    let (d, v) = gen_static_value_delayed(dat, block, from)?;
    if let Some(d) = d { d.run(dat); }
    Ok(v)
}

struct DelayedCompile<'a> {
    block: usize,
    from: &'a [Box<Token>],
}

impl<'a> DelayedCompile<'a> {
    pub fn run(self, dat: &'a mut CompileData) {
        dat.blocks[self.block].block = Some(compile_from_iter(dat, self.block, self.from));
    }
}

fn sanitize_regex(source: &str) -> String {
    let mut chars = source.chars();
    let mut out = String::with_capacity(source.len());

    loop {
        if let Some(c) = chars.next() {
            match c {
                '\\' => {
                    match chars.next() {
                        Some('/') => out.push('/'),
                        Some(e @ _) => {
                            out.push('\\');
                            out.push(e);
                        },
                        None => break,
                    }
                },
                _ => out.push(c),
            }

        } else { break; }
    }

    out
}

fn gen_static_value_delayed<'a, 'b>(dat: &'b mut CompileData, block: usize, from: &'a Token) -> Result<(Option<DelayedCompile<'a>>, StaticValue), String> {
    match from {
        // compile block and add to global functions
        &Token::Block(ref lines) => {
            let index = dat.blocks.len();
            dat.blocks.push(BlockDat::new(index, Some(block)));

            Ok((Some(DelayedCompile {
                block: index,
                from: &lines[..],
            }),
            StaticValue::Block{ index: index }))
        }
        // add to global regex list
        &Token::Regex(ref source) => {
            let source = sanitize_regex(source);
            let index = dat.regexs.entry(source).or_insert(dat.vars.next());
            let id = format!("_REGEX_{}", index);

            Ok((None, StaticValue::Regex{ id: id }))
        }
        // string literal
        &Token::StrLiteral(ref value) => Ok((None, StaticValue::Str{ value: value.clone() })),
        &Token::Name(ref id) => {
            if let Some(value) = dat.get_static(block, id) { Ok((None, value.clone())) }
            else { Err(format!("{} does not name a prior static value", id)) }
        },
        _ => Err(format!("{:?} is not a static value", from))
    }
}

fn compile_from_iter(dat: &mut CompileData, block: usize, toks: &[Box<Token>]) -> P<ast::Block> {
    let mut later = Vec::new();

    // find and compile statics
    for i in toks {
        match i {
            &box Token::Expr(Some(box Token::Name(ref op)), ref args) => match &op[..] {
                "stat" => {
                    // check name and value
                    if let [box Token::Name(ref name), box ref value] = args[..] {
                        let (d, v) = gen_static_value_delayed(dat, block, value).unwrap();
                        if let Some(d) = d { later.push(d); }
                        dat.blocks[block].statics.insert(name.clone(), v);  // TODO no unwraps
                    } else {
                        panic!(format!("\"stat\" expr \"stat <name> <value>\" has invalid args: {:?}", args))
                    }
                },
                "def" => {
                    let err = format!("\"def\" expr \"def <name> [<str|item|list> ...]\" has invalid args: {:?}", args);
                    // get the new defined tuple/enum's name (first arg)
                    if let [box Token::Name(ref name), ..] = args[..] {
                        let mut parts = Vec::new();
                        for i in &args[1..] {
                            // for each type in the tuple (remaining args)
                            if let &box Token::Name(ref ty) = i {
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
                        if dat.defs.iter().find(|v| v.0 == &name[..]).is_some() { panic!(format!("{} is already defined", name)); }
                        dat.defs.push((name.clone(), parts));
                    } else {
                        panic!(err);
                    }
                },
                _ => (),
            },
            _ => (),
        }
    }

    for d in later { d.run(dat); }

    let mut code = BlockBuilder::new();
    code = code.stmt().let_()
        .mut_id("_at")
        .expr().id("_start");
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
            &box Token::Expr(Some(ref op), ref args) => code = code.with_stmts(
                compile_expr(dat, block, op.as_ref(), args).into_iter()),
            &box Token::Comment(_) => (),
            _ => panic!(format!("{:?} is not a valid program line", t)),
        }
    }

    code = code.stmt().expr().ok()
    .tuple()
        .expr().id("_at")
        .expr().id("_out")
        .build();

    code.build()
}

pub fn compile(toks: &[Box<Token>]) {
    let mut dat = CompileData::new();
    dat.blocks.push(BlockDat::new(0, None));
    dat.blocks[0].block = Some(compile_from_iter(&mut dat, 0, toks));

    let mut items: Vec<P<ast::Item>> = Vec::new();

    let mut tokenum = ItemBuilder::new()
    .attr()
        .list("derive")
        .words(&["Clone", "Debug"])
        .build()
    .pub_().enum_("Token");
    for (id, parts) in dat.defs {
        tokenum =  if let Some((first, rest)) = parts.split_first() {
            let mut vs = tokenum.tuple(id)
                .build_ty(first.ty());
            for p in rest { vs = vs.with_ty(p.ty()); }
            vs.build()
        } else {
            tokenum.id(id)
        }
    }
    items.push(tokenum.build());

    let mainfn = ItemBuilder::new().pub_().fn_("parse")
        .arg()
            .id("input")
            .ty().ref_().ty().id("str")
        .return_()
            .result()
                .path()
                    .global()
                    .id("std")
                    .id("vec")
                    .segment("Vec")
                        .ty().box_().id("Token")
                        .build()
                    .build()
                .path()
                    .global()
                    .id("tredlib")
                    .id("ParseErr")
        .build().block()
        .stmt().expr().match_()
            .call()
                .id(&dat.blocks[0].id)
                .arg().usize(0)
                .arg().id("input")
                .build()
            .arm()
                .pat().ok().tuple()
                    .pat().wild()
                    .pat().id("tree")
                    .build()
                .body().ok().id("tree")
            .arm()
                .pat().err().id("err")
                .body().err().id("err")
            .build()
        .build();
    items.push(mainfn);

    let mut regexmac = ItemBuilder::new().mac().path().id("lazy_static").build();
    let mut regexs: Vec<(&String, &usize)> = dat.regexs.iter().collect();
    regexs.sort_by(|a, b| a.1.cmp(b.1));
    for (source, index) in regexs {
        regexmac = regexmac
        .expr().id("static")
        .expr().id("ref")
        .expr().assign()
            .type_()
                .id(format!("_REGEX_{}", index))
                .path().global().ids(&["tredlib", "regex", "Regex"]).build()
            .method_call("unwrap")
                .call()
                    .path().global().ids(&["tredlib", "regex", "Regex", "new"]).build()
                    .arg().str(&(String::from("^") + &source[..])[..])
                    .build()
                .build()
        .expr().id(";");
    }
    items.push(regexmac.build());

    for b in dat.blocks {
        items.push(ItemBuilder::new().fn_(b.id)
            .arg()
                .id("_start")
                .ty().usize()
            .arg()
                .id("_text")
                .ty().ref_()
                    .ty().id("str")
            .return_()
                .result()
                    .tuple()
                        .ty().usize()
                        .ty().path()
                            .global()
                            .id("std")
                            .id("vec")
                            .segment("Vec")
                                .ty().box_().id("Token")
                                .build()
                            .build()
                        .build()
                    .path()
                        .global()
                        .id("tredlib")
                        .id("ParseErr")
            .build()
        .build(b.block.unwrap()))
    }

    for f in items {
        println!("{}", syntax::print::pprust::item_to_string(f.deref()));
    }
    // TODO enums, funcs, and final output
}