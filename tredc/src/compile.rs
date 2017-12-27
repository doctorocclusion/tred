use quote::{Ident, Tokens};

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

const STRING_TY: &'static str = "::std::string::String";
fn option_box(ident: &str) -> Ident {
    Ident::new(format!("::std::option::Option<::std::boxed::Box<{}>>", ident))
}
fn vec_box(ident: &str) -> Ident {
    Ident::new(format!("::std::vec::Vec<::std::boxed::Box<{}>>", ident))
}

impl DefPart {
    pub fn ty(&self) -> Ident {
        match self {
            &DefPart::STR => Ident::new(STRING_TY),
            &DefPart::ITEM => option_box("Token"),
            &DefPart::LIST => vec_box("Token")
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

    pub fn next_name(&self, prefix: &str) -> Ident {
        Ident::new(format!("{}{}", prefix, self.next()))
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
    pub index: usize,
    pub parent: Option<usize>,
    pub statics: HashMap<String, StaticValue>,
    pub dyns: HashMap<String, DynValue>,
    pub active_into: Vec<IntoRec>,
    pub block: Option<Tokens>,
}

impl BlockDat {
    pub fn new(index: usize, parent: Option<usize>) -> BlockDat {
        BlockDat {
            index: index,
            parent: parent,
            statics: HashMap::new(),
            dyns: HashMap::new(),
            active_into: Vec::new(),
            block: None,
        }
    }

    pub fn gen_append(&self, vars: &SecNext, vec: &Ident) -> Tokens {
        let last = self.active_into.len() - 1;
        let intos = self.active_into
            .iter()
            .enumerate()
            .filter_map(|(i, into)| into.append_part(vec, i == last));
        quote! { {#(#intos)*} }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntoRec {
    List(Ident),
    Once(Ident, bool),
}

impl IntoRec {
    pub fn as_vec(&self, leave: bool) -> Tokens {
        match (self, leave) {
            (&IntoRec::Once(ref var, false), _) => quote! { #var.iter().cloned().collect() },
            (&IntoRec::Once(ref var, true), true)
            | (&IntoRec::List(ref var), true) => quote! { #var.clone() },
            (&IntoRec::Once(ref var, true), false)
            | (&IntoRec::List(ref var), false) => quote! { #var },
        }
    }

    pub fn as_opt(&self, leave: bool) -> Tokens {
        match (self, leave) {
            (&IntoRec::Once(ref var, false), true) => quote! { #var.clone() },
            (&IntoRec::Once(ref var, true), true) 
            | (&IntoRec::List(ref var), true) => quote! { #var.iter().cloned().next() },
            (&IntoRec::Once(ref var, false), false) => quote! { #var },
            (&IntoRec::Once(ref var, true), false)
            | (&IntoRec::List(ref var), false) => quote! { var.into_iter().next() }
        }
    }

    pub fn append_part(&self, vec: &Ident, only: bool) -> Option<Tokens> {
        Some(match *self {
            IntoRec::Once(ref var, list) => {
                match (list, only) {
                    (true, true) => quote! { #var.append(#vec); },
                    (true, false) => quote! { #var.extend(#vec.iter().cloned()); },
                    (false, true) => quote! { #var = #vec.pop().or(#var); },
                    (false, false) => quote! { #var = #vec.last().cloned().or(#var); },
                }
            },
            IntoRec::List(ref var) => {
                if only {
                    quote! { #var.append(#vec); }

                } else {
                    quote! { #var.extend(#vec.iter().cloned()); }
                }
            }
        })
    }
}

#[derive(Debug)]
enum DynValue {
    Val (Value),
    Capture {dat: CaptureDat},
    IntoVal (IntoRec)
}

impl DynValue {
    pub fn deref(&self, vars: &SecNext, code: &mut Tokens) -> Result<Value, String> {
        match self {
            &DynValue::Val(ref val) => Ok(val.clone()),
            &DynValue::Capture{dat: ref cap} => {
                let name = vars.next_name("_cap_");
                let expr = cap.eval_ref();
                code.append(quote! { let #name = #expr; });
                Ok(Value::Str {var_name: name})
            },
            _ => Err(format!("{:?} is not a matchable value", self)),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Static (StaticValue),
    Str {var_name: Ident},
}

impl Value {
    pub fn gen_match(&self, _pre: &mut Tokens) -> Tokens {
        match self {
            &Value::Static(ref stat) => stat.gen_match(),
            &Value::Str{ref var_name} => quote! { _tredgen_match_str!(_at, _text, #var_name) }
        }
    }
}

#[derive(Debug, Clone)]
enum StaticValue {
    Regex {id: Ident},
    Str {value: String},
    Block {index: usize},
}

impl StaticValue {
    pub fn into_val(self) -> Value {
        Value::Static(self)
    }

    pub fn gen_match(&self) -> Tokens {
        match self {
            &StaticValue::Str{ref value} => {
                let string = unescape(value).unwrap();
                quote! { _tredgen_match_str!(_at, _text, #string) }
            },
            &StaticValue::Regex{ref id} => quote! { _tredgen_match_regex!(_at, _text, #id) },
            &StaticValue::Block{ref index} => {
                let id = Ident::new(format!("_blockfn_{}", index));
                quote! { #id(_at, _text) }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CaptureDat {
    pub acc_name: Option<Ident>,
    pub start_name: Ident,
    pub is_ended: bool,
}

impl CaptureDat {
    pub fn start(vars: &SecNext, code: &mut Tokens) -> CaptureDat {
        let dat = CaptureDat {
            acc_name: None,
            start_name: vars.next_name("_start_"),
            is_ended: false,
        };

        {
            let name = &dat.start_name;
            code.append(quote! { let mut #name: usize; });
        }

        dat
    }

    pub fn gen_start(&mut self) -> Tokens {
        self.is_ended = false;

        let name = &self.start_name;
        quote! { #name = _at; }
    }

    pub fn gen_end(&mut self, vars: &SecNext) -> Tokens {
        let push = self.acc_name.is_some();

        if !push { self.acc_name = Some(vars.next_name("_acc_")); }
        let name = self.acc_name.as_ref().unwrap();
        let start = &self.start_name;

        self.is_ended = true;

        if push {
            quote! { #name.push_str(&_text[#start.._at]); }
        } else {
            quote! { let mut #name = _text[#start.._at].to_owned(); }
        }
    }

    pub fn eval_ref(&self) -> Tokens {
        let start = &self.start_name;
        match (self.is_ended, &self.acc_name) {
            (false, &Some(ref acc)) => quote! { &(#acc.clone() + &_text[#start.._at]) },
            (true, &Some(ref  acc)) => quote! { #acc.as_str() },
            (false, &None) => quote! { &_text[#start.._at] },
            _ => panic!("Illegal capture state")
        }
    }

    pub fn eval_own(&self) -> Tokens {
        let start = &self.start_name;
        match (self.is_ended, &self.acc_name) {
            (false, &Some(ref acc)) => quote! { #acc.clone() + &_text[#start.._at] },
            (true, &Some(ref  acc)) => quote! { #acc.clone() },
            (false, &None) => quote! { _text[#start.._at].to_owned() },
            _ => panic!("Illegal capture state")
        }
    }
}

fn gen_expr_vals(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Token>>) -> Tokens {
    let mac = Ident::new(mac);
    let err = format!("\"{}\" expr \"{} <value> [<value>...]\" has invalid args: {:?}", op, op, args);
    let mut out = Tokens::new();

    if args.len() < 1 {
        panic!(err); // TODO no panics
    }

    let res_vec = dat.vars.next_name("_resvec_");
    let other_args: Vec<_> = args.iter()
        .map(|b| gen_value(dat, blocki, b.as_ref(), &mut out)
            .expect(&err)
            .gen_match(&mut out))
        .collect();
    let block = &dat.blocks[blocki];
    let block = block.gen_append(&dat.vars, &res_vec);
    out.append(quote! { #mac!(_at, _text, #res_vec, #block #(, #other_args)*); });

    out
}

fn gen_expr_val(mac: &str, op: &str, dat: &mut CompileData, blocki: usize, args: &Vec<Box<Token>>) -> Tokens {
    let mac = Ident::new(mac);
    let err = format!("\"{}\" expr \"{} <value>\" has invalid args: {:?}", op, op, args);

    if let [box ref val] = args[..] {
        let mut out = Tokens::new();

        let val = gen_value(dat, blocki, val, &mut out).expect(&err);
        let block = &dat.blocks[blocki];
        let res_vec = dat.vars.next_name("_resvec_");
        let block = block.gen_append(&dat.vars, &res_vec);
        let mat = val.gen_match(&mut out);

        out.append(quote! { #mac!(_at, _text, #res_vec, #block, #mat); });

        out
    } else {
        panic!(err);
    }
}

fn gen_token_output(dat: &CompileData, block: usize, def: DefPart, item: &Token) -> Result<Tokens, String>  {
    match (def, item) {
        (DefPart::ITEM, &Token::Tuple(Some(box Token::Name(ref name)), ref parts)) => {
            let part_defs = dat.defs.iter().find(|v| v.0 == &name[..]);
            if part_defs.is_none() { return Err(format!("{} is an unknown definition", name)); }
            let part_defs = &part_defs.unwrap().1;

            if part_defs.len() != parts.len() { return Err(format!("{} has {} members, only {} were supplied", name, part_defs.len(), parts.len())); }

            let name = Ident::new(name.as_str());
            if !part_defs.is_empty() {
                let params: Result<Vec<_>, _> = (0..part_defs.len()).map(|i| 
                    gen_token_output(dat, block, part_defs[i], parts[i].as_ref()))
                    .collect();
                match params {
                    Ok(params) => Ok(quote! { Some(::std::boxed::Box::new(Token::#name(#(#params),*))) }),
                    Err(e) => Err(e),
                }
            } else {
                Ok(quote! { Some(::std::boxed::Box::new(Token::#name)) })
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
                    Some(&StaticValue::Str {value: ref val}) => {
                        let val = unescape(val).unwrap();
                        Ok(quote! { #val })
                    },
                    Some(v @ _) => Err(format!("{} ({:?}) is static but not a string", name, v)),
                    None => Err(format!("{} does not name a value", name)),
                }
            }


        },
        (DefPart::STR, &Token::StrLiteral(ref val)) => {
            let val = unescape(val).unwrap();
            Ok(quote! { #val })
        },
        _ => Err(format!("{:?} is not a valid as a {:?} member", item, def))
    }
}

fn compile_name_expr(dat: &mut CompileData, blocki: usize, op: &String, args: &Vec<Box<Token>>) -> Tokens {
    let vars = dat.vars.clone();

    match &op[..] {
        // def expression (already handled)
        "def" => Tokens::new(),
        // stat expression (already handled)
        "stat" => Tokens::new(),
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
                        let mut out = Tokens::new();
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
                        return Tokens::new();
                    },
                    Some(v @ _) => panic!(format!("Can not append into \"{}\", already has value: {:?}", n, v)),
                    None => name = n,
                }
            } else {
                panic!(err);
            }

            let exporting = name == "export";
            let id = if exporting { Ident::new("_out") } else { vars.next_name("_intolist_") };
            let rec = IntoRec::List(id.clone());
            block.active_into.push(rec.clone());
            block.dyns.insert(name.clone(), DynValue::IntoVal(rec));
            if exporting { Tokens::new() } else { quote! { let mut #id = ::std::vec::Vec::new(); } }
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
                        return Tokens::new();
                    },
                    Some(v @ _) => panic!(format!("Can not assign into \"{}\", already has value: {:?}", n, v)),
                    None => name = n,
                }
            } else {
                panic!(err);
            }

            let exporting = name == "export";
            let id = if exporting { Ident::new("_out") } else { vars.next_name("_intoonce_") };
            let rec = IntoRec::Once(id.clone(), exporting);
            block.active_into.push(rec.clone());
            block.dyns.insert(name.clone(), DynValue::IntoVal(rec));
            if exporting { Tokens::new() } else { quote! { let mut #id = None; } }
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

                Tokens::new()
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
                let expr = gen_token_output(dat, blocki, DefPart::ITEM, item).expect(&err);
                quote! { _out.extend(#expr); }
            } else {
                panic!(err);
            }  
        },
        // no other expressions
        op @ _ => panic!(format!("Unknown operation: {}", op)),
    }
}
 
fn compile_expr(dat: &mut CompileData, block: usize, op: &Token, args: &Vec<Box<Token>>) -> Tokens {
    let mut out = Tokens::new();

    if let Ok(val) = gen_value(dat, block, op, &mut out) {
        let res_vec = dat.vars.next_name("_resvec_");
        let block = dat.blocks[block].gen_append(&dat.vars, &res_vec);
        let mat = val.gen_match(&mut out);
        out.append(quote! { _tredgen_append!(_at, #res_vec, #block, #mat?); });
        out
    } else if let &Token::Name(ref name) = op {
        compile_name_expr(dat, block, name, args)
    } else {
        panic!(format!("{:?} is not a matchable value or operation", op)) // TODO no panic
    }
}

fn gen_value<'a>(dat: &'a mut CompileData, block: usize, from: &Token, prefix: &mut Tokens) -> Result<Value, String> {
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
            let id = Ident::new(format!("_REGEX_{}", index));

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

fn compile_from_iter(dat: &mut CompileData, block: usize, toks: &[Box<Token>]) -> Tokens {
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

    let mut code = quote! {
        let mut _at = _start;
        let mut _out = ::std::vec::Vec::new();
    };

    // actually compile
    for t in toks {
        match t {
            &box Token::Expr(Some(ref op), ref args) => code.append(
                compile_expr(dat, block, op.as_ref(), args)),
            &box Token::Comment(_) => (),
            _ => panic!(format!("{:?} is not a valid program line", t)),
        }
    }

    code.append(quote! { Ok((_at, _out)) });
    code
}

pub fn compile(toks: &[Box<Token>]) {
    let mut dat = CompileData::new();
    dat.blocks.push(BlockDat::new(0, None));
    dat.blocks[0].block = Some(compile_from_iter(&mut dat, 0, toks));

    let mut items = Tokens::new();

    let varis = dat.defs.into_iter().map(|(id, parts)| {
        let id = Ident::new(id);
        if parts.is_empty() { 
            quote! { #id }
        } else {
            let tys = parts.iter().map(|p| p.ty());
            quote! { #id(#(#tys),*) } 
        }
    });
    items.append(quote! {
        #[derive(Clone, Debug)]
        pub enum Token {
            #(#varis,)*
        }
    });

    let vbt_ty = vec_box("Token");
    let main_block = Ident::new(format!("_blockfn_{}", dat.blocks[0].index));
    items.append(quote! {
        pub fn parse(input: &str) -> Result<#vbt_ty, ::tredlib::ParseErr> {
            match #main_block(0usize, input) {
                Result::Ok((_, tree)) => Result::Ok(tree),
                Result::Err(err) => Result::Err(err),
            }
        }
    });

    let reg_defs = dat.regexs.into_iter().map(|(source, index)| {
        let id = Ident::new(format!("_REGEX_{}", index));
        let source = format!("^{}", source);
        quote! { static ref #id: ::tredlib::regex::Regex = ::tredlib::regex::Regex::new(#source).unwrap(); }
    });
    items.append(quote! {
        lazy_static! {
            #(#reg_defs)*
        }
    });

    for b in dat.blocks {
        let id = Ident::new(format!("_blockfn_{}", b.index));
        let body = b.block.unwrap();
        items.append(quote! {
            fn #id(_start: usize, _text: &str) -> Result<(usize, #vbt_ty), ::tredlib::ParseErr> {
                #body
            }
        });
    }

    println!("{}", items);
    // TODO enums, funcs, and final output
}