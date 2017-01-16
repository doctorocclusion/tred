#[derive(Clone, Debug)]
pub enum Token {
    True,
    Map(::std::vec::Vec<::std::boxed::Box<Token>>),
    MapEntry(::std::string::String,
             ::std::option::Option<::std::boxed::Box<Token>>),
    String(::std::string::String),
    Null,
    False,
    Array(::std::vec::Vec<::std::boxed::Box<Token>>),
    Number(::std::string::String),
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
              _regex_1: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]+").unwrap()
              ; static ref
              _regex_42: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[1-9]").unwrap()
              ; static ref
              _regex_4: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^u[\\da-fA-F]{6}").unwrap()
              ; static ref
              _regex_2: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[^\\\\\"]").unwrap()
              ; static ref
              _regex_0: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^[\\s\\n\\r]*").unwrap()
              ; static ref
              _regex_44: ::tredlib::regex::Regex =
    ::tredlib::regex::Regex::new("^\\d").unwrap()
              ;);
fn _blockfn_0(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_61 , { _out.append(_resvec_61) } ,
                     match _blockfn_8(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_1(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_some!(_at , _text , _resvec_7 , { } , _blockfn_9(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_2(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_8 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_9: usize;
    _start_9 = _at;
    _tredgen_append!(_at , _resvec_10 , { } ,
                     match _blockfn_1(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::String(::std::string::String::from(&_text[_start_9.._at])))));
    _tredgen_append!(_at , _resvec_11 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
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
    _tredgen_append!(_at , _resvec_12 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _start_13: usize;
    _start_13 = _at;
    _tredgen_append!(_at , _resvec_14 , { } ,
                     match _blockfn_1(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _acc_15 = ::std::string::String::from(&_text[_start_13.._at]);
    _tredgen_append!(_at , _resvec_16 , { } ,
                     match _tredgen_match_str!(_at , _text , "\"") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_17 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_18 , { } ,
                     match _tredgen_match_str!(_at , _text , ":") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_19 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _intoonce_20 = ::std::option::Option::None;
    _tredgen_append!(_at , _resvec_21 ,
                     { _intoonce_20 = _resvec_21.pop().or(_intoonce_20) } ,
                     match _blockfn_8(_at, _text) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::MapEntry(_acc_15.clone(),
                                                                                   _intoonce_20.clone()))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_4(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_22 , { } ,
                     match _tredgen_match_str!(_at , _text , "{") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_23 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _intolist_24 = ::std::vec::Vec::new();
    _tredgen_some!(_at , _text , _resvec_28 ,
                   { _intolist_24.append(_resvec_28) } ,
                   _blockfn_3(_at, _text) , _blockfn_11(_at, _text));
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Map(_intolist_24.clone()))));
    _tredgen_append!(_at , _resvec_29 , { _intolist_24.append(_resvec_29) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_30 , { _intolist_24.append(_resvec_30) } ,
                     match _tredgen_match_str!(_at , _text , "}") {
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
    _tredgen_append!(_at , _resvec_31 , { } ,
                     match _tredgen_match_str!(_at , _text , "[") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_32 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    let mut _intolist_33 = ::std::vec::Vec::new();
    _tredgen_some!(_at , _text , _resvec_37 ,
                   { _intolist_33.append(_resvec_37) } ,
                   _blockfn_8(_at, _text) , _blockfn_12(_at, _text));
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Array(_intolist_33.clone()))));
    _tredgen_append!(_at , _resvec_38 , { _intolist_33.append(_resvec_38) } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_39 , { _intolist_33.append(_resvec_39) } ,
                     match _tredgen_match_str!(_at , _text , "]") {
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
    let mut _start_40: usize;
    _start_40 = _at;
    _tredgen_option!(_at , _text , _resvec_41 , { } ,
                     _tredgen_match_str!(_at , _text , "-"));
    _tredgen_or!(_at , _text , _resvec_46 , { } ,
                 _tredgen_match_str!(_at , _text , "0") ,
                 _blockfn_13(_at, _text));
    _tredgen_option!(_at , _text , _resvec_50 , { } ,
                     _blockfn_14(_at, _text));
    _tredgen_option!(_at , _text , _resvec_55 , { } ,
                     _blockfn_15(_at, _text));
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Number(::std::string::String::from(&_text[_start_40.._at])))));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_7(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_59 , { _out.append(_resvec_59) } ,
                 _blockfn_16(_at, _text) , _blockfn_17(_at, _text) ,
                 _blockfn_18(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_8(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_60 , { _out.append(_resvec_60) } ,
                 _blockfn_2(_at, _text) , _blockfn_4(_at, _text) ,
                 _blockfn_5(_at, _text) , _blockfn_7(_at, _text) ,
                 _blockfn_6(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_9(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_6 , { } ,
                 _tredgen_match_regex!(_at , _text , _regex_2) ,
                 _blockfn_10(_at, _text));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_10(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_3 , { } ,
                     match _tredgen_match_str!(_at , _text , "\\") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_or!(_at , _text , _resvec_5 , { } ,
                 _tredgen_match_str!(_at , _text , "\"") ,
                 _tredgen_match_str!(_at , _text , "\\") ,
                 _tredgen_match_str!(_at , _text , "/") ,
                 _tredgen_match_str!(_at , _text , "b") ,
                 _tredgen_match_str!(_at , _text , "n") ,
                 _tredgen_match_str!(_at , _text , "f") ,
                 _tredgen_match_str!(_at , _text , "n") ,
                 _tredgen_match_str!(_at , _text , "r") ,
                 _tredgen_match_str!(_at , _text , "t") ,
                 _tredgen_match_regex!(_at , _text , _regex_4));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_11(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_25 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_26 , { } ,
                     match _tredgen_match_str!(_at , _text , ",") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_27 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_12(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_34 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_35 , { } ,
                     match _tredgen_match_str!(_at , _text , ",") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_append!(_at , _resvec_36 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_0) {
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
    _tredgen_append!(_at , _resvec_43 , { } ,
                     match _tredgen_match_regex!(_at , _text , _regex_42) {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_some!(_at , _text , _resvec_45 , { } ,
                   _tredgen_match_regex!(_at , _text , _regex_44));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_14(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_47 , { } ,
                     match _tredgen_match_str!(_at , _text , ".") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _tredgen_many!(_at , _text , _resvec_49 , { } ,
                   _tredgen_match_regex!(_at , _text , _regex_44));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_15(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(_at , _text , _resvec_51 , { } ,
                 _tredgen_match_str!(_at , _text , "e") ,
                 _tredgen_match_str!(_at , _text , "E"));
    _tredgen_or!(_at , _text , _resvec_52 , { } ,
                 _tredgen_match_str!(_at , _text , "+") ,
                 _tredgen_match_str!(_at , _text , "-") ,
                 _tredgen_match_str!(_at , _text , ""));
    _tredgen_many!(_at , _text , _resvec_54 , { } ,
                   _tredgen_match_regex!(_at , _text , _regex_44));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_16(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_56 , { } ,
                     match _tredgen_match_str!(_at , _text , "true") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::True)));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_17(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_57 , { } ,
                     match _tredgen_match_str!(_at , _text , "false") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::False)));
    ::std::result::Result::Ok((_at, _out))
}
fn _blockfn_18(_start: usize, _text: &str)
 ->
     ::std::result::Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>),
                           ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at , _resvec_58 , { } ,
                     match _tredgen_match_str!(_at , _text , "null") {
    ::std::result::Result::Ok(value) => value,
    ::std::result::Result::Err(err) =>
    return ::std::result::Result::Err(::std::convert::From::from(err)),
});
    _out.extend(::std::option::Option::Some(::std::boxed::Box::new(Token::Null)));
    ::std::result::Result::Ok((_at, _out))
}
