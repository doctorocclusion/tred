#[derive(Clone, Debug)]
pub enum Token {
    Tuple(::std::option::Option<::std::boxed::Box<Token>>,
          ::std::vec::Vec<::std::boxed::Box<Token>>),
    Regex(::std::string::String),
    Block(::std::vec::Vec<::std::boxed::Box<Token>>),
    Comment(::std::string::String),
    Expr(::std::option::Option<::std::boxed::Box<Token>>,
         ::std::vec::Vec<::std::boxed::Box<Token>>),
    StrLiteral(::std::string::String),
    Name(::std::string::String),
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
              _regex_4: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\w_]+").unwrap()
              ; static ref
              _regex_33: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^\\s*").unwrap()
              ; static ref
              _regex_25: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^([^/\\\\]|(\\\\.))*").unwrap()
              ; static ref
              _regex_36: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[^\\n]*").unwrap()
              ; static ref
              _regex_0: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]*").unwrap()
              ; static ref
              _regex_20: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^([^\"\\\\]|(\\\\.))*").unwrap()
              ; static ref
              _regex_1: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]+").unwrap()
              ;);
fn _blockfn_0(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_all!(_at , _text , _resvec_48 , { _out.append(_resvec_48) } ,
                  _blockfn_9(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_1(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_not!(_at , _text , _resvec_2 , { } ,
                  _tredgen_match_str!(_at , _text , "_"));
    let mut _start_3: usize;
    _start_3 = _at;
    _tredgen_append!(_at , _resvec_5 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_4) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Name(::std::string::String::from(&_text[_start_3.._at])))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_2(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_not!(_at , _text , _resvec_6 , { } ,
                  _tredgen_match_str!(_at , _text , "_"));
    let mut _start_7: usize;
    _start_7 = _at;
    _tredgen_append!(_at , _resvec_9 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_4) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _acc_10 = ::std::string::String::from(&_text[_start_7.._at]);
    let mut _intolist_11 = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_12 , { _intolist_11.append(_resvec_12) } ,
                     match _tredgen_match_str!(_at , _text , "(") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_13 , { _intolist_11.append(_resvec_13) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text , _resvec_15 ,
                   { _intolist_11.append(_resvec_15) } ,
                   _blockfn_10(_at, _text) ,
                   _tredgen_match_regex!(_at , _text , _regex_1));
    _tredgen_append!(_at , _resvec_16 , { _intolist_11.append(_resvec_16) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_17 , { _intolist_11.append(_resvec_17) } ,
                     match _tredgen_match_str!(_at , _text , ")") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Tuple(::std::option::Option::Some(::std::boxed::Box::new(Token::Name(_acc_10.clone()))),
                                                                                _intolist_11.clone()))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_3(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_18 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_19: usize;
    _start_19 = _at;
    _tredgen_append!(_at , _resvec_21 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_20) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::StrLiteral(::std::string::String::from(&_text[_start_19.._at])))));
    _tredgen_append!(_at , _resvec_22 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
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
    _tredgen_append!(_at , _resvec_23 , { } ,
                     match _tredgen_match_str!(_at , _text , "/") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_24: usize;
    _start_24 = _at;
    _tredgen_append!(_at , _resvec_26 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_25) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Regex(::std::string::String::from(&_text[_start_24.._at])))));
    _tredgen_append!(_at , _resvec_27 , { } ,
                     match _tredgen_match_str!(_at , _text , "/") {
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
    let mut _intolist_28 = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_29 , { _intolist_28.append(_resvec_29) } ,
                     match _tredgen_match_str!(_at , _text , "{") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text , _resvec_30 ,
                   { _intolist_28.append(_resvec_30) } ,
                   _blockfn_9(_at, _text));
    _tredgen_append!(_at , _resvec_31 , { _intolist_28.append(_resvec_31) } ,
                     match _tredgen_match_str!(_at , _text , "}") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Block(_intolist_28.clone()))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_6(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_32 , { } ,
                     match _tredgen_match_str!(_at , _text , "//") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_34 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_33) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_35: usize;
    _start_35 = _at;
    _tredgen_append!(_at , _resvec_37 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_36) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Comment(::std::string::String::from(&_text[_start_35.._at])))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_7(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_38 , { _out.append(_resvec_38) } ,
                 _blockfn_3(_at, _text) , _blockfn_4(_at, _text) ,
                 _blockfn_5(_at, _text) , _blockfn_2(_at, _text) ,
                 _blockfn_1(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_8(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    let mut _intoonce_39 = ::std::option::Option::None;
    _tredgen_append!(_at , _resvec_40 ,
                     { _intoonce_39 = _resvec_40.pop().or(_intoonce_39) } ,
                     match _blockfn_7(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _intolist_41 = ::std::vec::Vec::new();
    _tredgen_some!(_at , _text , _resvec_44 ,
                   { _intolist_41.append(_resvec_44) } ,
                   _blockfn_11(_at, _text));
    _tredgen_append!(_at , _resvec_45 , { _intolist_41.append(_resvec_45) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_46 , { _intolist_41.append(_resvec_46) } ,
                     match _tredgen_match_str!(_at , _text , ";") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Expr(_intoonce_39.clone(),
                                                                               _intolist_41.clone()))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_9(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_47 , { _out.append(_resvec_47) } ,
                 _tredgen_match_regex!(_at , _text , _regex_1) ,
                 _blockfn_6(_at, _text) , _blockfn_8(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_10(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_14 , { _out.append(_resvec_14) } ,
                     match _blockfn_7(_at, _text) {
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
    _tredgen_append!(_at , _resvec_42 , { _out.append(_resvec_42) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_1) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_43 , { _out.append(_resvec_43) } ,
                     match _blockfn_7(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
