#[derive(Clone, Debug)]
pub enum Token {
    Expr(::std::option::Option<::std::boxed::Box<Token>>,
         ::std::vec::Vec<::std::boxed::Box<Token>>),
    Comment(::std::string::String),
    Name(::std::string::String),
    Tuple(::std::option::Option<::std::boxed::Box<Token>>,
          ::std::vec::Vec<::std::boxed::Box<Token>>),
    Regex(::std::string::String),
    StrLiteral(::std::string::String),
    Block(::std::vec::Vec<::std::boxed::Box<Token>>),
}
pub fn parse(input: &str)
 ->
     ::std::result::Result<::std::vec::Vec<::std::boxed::Box<Token>>,
                           ::tredlib::ParseErr> {
    match _blockfn_0(0usize, input) {
        ::std::result::Result::Ok((_, tree)) =>
        ::std::result::Result::Ok(tree),
        ::std::result::Result::Err(err) => ::std::result::Result::Err(err),
    }
}
lazy_static! (static ref
              _regex_7: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("\"").unwrap()
              ; static ref
              _regex_1: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("[\\s\\n\\r]+").unwrap()
              ; static ref
              _regex_0: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("[\\s\\n\\r]*").unwrap()
              ; static ref
              _regex_9: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("[^\"]").unwrap()
              ; static ref
              _regex_15: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("[^\\n]*").unwrap()
              ; static ref
              _regex_3: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("[\\w_]+").unwrap()
              ; static ref
              _regex_12: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new(".").unwrap()
              ; static ref
              _regex_13: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("\\s*").unwrap()
              ;);
fn _blockfn_0(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_all!(_at , _text ,
                  (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                  , _blockfn_11(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_1(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_2(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_1) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_3(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_not!(_at , _text ,
                  (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                  , _tredgen_match_str!(_at , _text , "_"));
    let mut _start_2: usize;
    _start_2 = _at;
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_3) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_4(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_not!(_at , _text ,
                  (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                  , _tredgen_match_str!(_at , _text , "_"));
    let mut _start_4: usize;
    _start_4 = _at;
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_3) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _acc_6 = ::std::string::String::from(&_text[_start_4.._at]);
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "(") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_1(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text ,
                   (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                   , _blockfn_12(_at, _text) , _blockfn_2(_at, _text));
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_1(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , ")") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_5(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_7) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_8: usize;
    _start_8 = _at;
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_9) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_7) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_6(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "/") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_11: usize;
    _start_11 = _at;
    _tredgen_some!(_at , _text ,
                   (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                   , _blockfn_13(_at, _text));
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "/") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_7(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "{") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text ,
                   (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                   , _blockfn_11(_at, _text));
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "}") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_8(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , "//") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_13) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_14: usize;
    _start_14 = _at;
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_15) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_9(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text ,
                 (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                 , _blockfn_5(_at, _text) , _blockfn_6(_at, _text) ,
                 _blockfn_7(_at, _text) , _blockfn_4(_at, _text) ,
                 _blockfn_3(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_10(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_9(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text ,
                   (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                   , _blockfn_14(_at, _text));
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_1(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_str!(_at , _text , ";") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_11(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text ,
                 (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                 , _blockfn_2(_at, _text) , _blockfn_8(_at, _text) ,
                 _blockfn_10(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_12(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_9(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_13(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_not!(_at , _text ,
                  (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                  , _tredgen_match_str!(_at , _text , "/"));
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _tredgen_match_regex!(_at , _text , _regex_12) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_14(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_2(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at ,
                     (|_vec: &mut ::std::vec::Vec<::std::boxed::Box<Token>>| -> ()
     { _out.append(_vec) })
                     ,
                     match _blockfn_9(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
