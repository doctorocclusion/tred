#[derive(Clone, Debug)]
pub enum Token {
    Regex(::std::string::String),
    Expr(::std::option::Option<::std::boxed::Box<Token>>,
         ::std::vec::Vec<::std::boxed::Box<Token>>),
    Tuple(::std::option::Option<::std::boxed::Box<Token>>,
          ::std::vec::Vec<::std::boxed::Box<Token>>),
    Name(::std::string::String),
    StrLiteral(::std::string::String),
    Comment(::std::string::String),
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
              _regex_4: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\w_]+").unwrap()
              ; static ref
              _regex_0: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]*").unwrap()
              ; static ref
              _regex_1: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]+").unwrap()
              ; static ref
              _regex_19: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^([^\"\\\\]|(\\\\.))*").unwrap()
              ;);
fn _blockfn_0(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
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
    _tredgen_some!(_at , _text , _resvec_14 ,
                   { _intolist_11.append(_resvec_14) } ,
                   _blockfn_4(_at, _text) ,
                   _tredgen_match_regex!(_at , _text , _regex_1));
    _tredgen_append!(_at , _resvec_15 , { _intolist_11.append(_resvec_15) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_16 , { _intolist_11.append(_resvec_16) } ,
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
    _tredgen_append!(_at , _resvec_17 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_18: usize;
    _start_18 = _at;
    _tredgen_append!(_at , _resvec_20 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_19) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::StrLiteral(::std::string::String::from(&_text[_start_18.._at])))));
    _tredgen_append!(_at , _resvec_21 , { } ,
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
    ::std::result::Result::Ok((_at, _out))
}
