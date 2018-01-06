#[macro_export]
macro_rules! _tredgen_append {
    ($pos:ident, $vec:ident, $out:expr, $x:expr) => {
        {
            let mut __r = $x;
            $pos = __r.0;
            let $vec = &mut __r.1;
            $out;
        }
    };
}

#[macro_export]
macro_rules! _tredgen_match_str {
    ($pos:ident, $text:ident, $x:expr) => {
        {
            let __tmp: &str = $x;
            let __len: usize = __tmp.len();
            if (&$text[$pos..]).starts_with(__tmp) {
                let mut __empty : ::std::vec::Vec<::std::boxed::Box<Token>> = ::std::vec::Vec::new();
                ::std::result::Result::Ok(($pos + __len, __empty))
            } else {
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos, msg: ::std::option::Option::Some(
                    format!("String \"{}\" did not match", __tmp)
                ), cause: ::std::vec::Vec::new()})
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_match_regex {
    ($pos:ident, $text:ident, $x:expr) => {
        {
            if let ::std::option::Option::Some((_, __end)) = $x.find(&$text[$pos..]) {
                let mut __empty : ::std::vec::Vec<::std::boxed::Box<Token>> = ::std::vec::Vec::new();
                ::std::result::Result::Ok(($pos + __end, __empty))
            } else {
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos, msg: ::std::option::Option::Some(
                    format!("Regex did not match")
                ), cause: ::std::vec::Vec::new()})
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_or {
    ($pos:ident, $text:ident, $vec:ident, $out:expr $(, $x:expr)+) => {
        {
            let mut __causes = ::std::vec::Vec::new();
            let mut __next = true;
            $(
                if __next { match $x {
                    ::std::result::Result::Ok(mut __res) => { _tredgen_append!($pos, $vec, $out, __res); __next = false; },
                    ::std::result::Result::Err(__c) => __causes.push(__c), 
                }}
            )+
            if __next { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos, msg: ::std::option::Option::None, cause: __causes}); }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_not {
    ($pos:ident, $text:ident, $vec:ident, $out:expr, $x:expr) => {
        if let ::std::result::Result::Ok(_) = $x { 
           return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos, msg: ::std::option::Option::None, cause: ::std::vec::Vec::new()});
        }
    };
}

#[macro_export]
macro_rules! _tredgen_outer {
    ($pos:ident, $text:ident, $vec:ident, $out:expr, $x:expr) => {
        if let ::std::result::Result::Ok(__res) = $x {
            _tredgen_append!($pos, $vec, $out, __res);
        }
    };
}

#[macro_export]
macro_rules! _tredgen_nested {
    ($pos:ident, $text:ident, $vec:ident, $out:expr, $x:expr) => {
        while let ::std::result::Result::Ok(__res) = $x {
            _tredgen_append!($pos, $vec, $out, __res);
        }
    };
}

#[macro_export]
macro_rules! _tredgen_option {
    ($pos:ident, $text:ident, $vec:ident, $out:expr, $x1:expr $(, $x2:expr)*) => {
        if let ::std::result::Result::Ok(mut __res) = $x1 {
            _tredgen_append!($pos, $vec, $out, __res);
        }
        $(else if let ::std::result::Result::Ok(mut __res) = $x2 {
            _tredgen_append!($pos, $vec, $out, __res);
        })*
    };
}

#[macro_export]
macro_rules! _tredgen_many {
    ($pos:ident, $text:ident, $vec:ident, $out:expr, $x1:expr $(, $x2:expr)*) => {
        {
            let mut __mark = false;
            loop {
                match $x1 {
                    ::std::result::Result::Ok(mut __res) => _tredgen_append!($pos, $vec, $out, __res),
                    ::std::result::Result::Err(__c) =>  
                        if !__mark { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos, msg: ::std::option::Option::None, cause: vec![__c]}); }
                        else { break; }
                }
                __mark = true;
                $(if let ::std::result::Result::Ok(mut __res) = $x2 {
                    _tredgen_append!($pos, $vec, $out, __res);
                } else {
                    break;
                })*
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_some {
    ($pos:ident, $text:ident, $vec:ident, $out:expr $(, $x:expr)+) => {
        loop {
            $(if let ::std::result::Result::Ok(mut __res) = $x {
                _tredgen_append!($pos, $vec, $out, __res);
            } else {
                break;
            })+
        }
    };
}

#[macro_export]
macro_rules! _tredgen_all {
    ($pos:ident, $text:ident, $vec:ident, $out:expr $(, $x:expr)+) => {
        while $pos < $text.len() {
            $(match $x {
                Ok(mut __res) => _tredgen_append!($pos, $vec, $out, __res),
                __e @ _ => return __e,
            })+
        }
    };
}