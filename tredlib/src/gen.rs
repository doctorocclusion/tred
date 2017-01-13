#[macro_export]
macro_rules! _tredgen_append {
    ($pos:ident, $out:expr, $x:expr) => {
        {
            let mut __r = $x;
            $pos = __r.0;
            $out(&mut __r.1);
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
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos})
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_match_regex {
    ($pos:ident, $text:ident, $x:expr) => {
        {
            if let ::std::option::Option::Some((_, end)) = $x.find(&$text[$pos..]) {
                ::std::result::Result::Ok(($pos + end, ::std::vec::Vec::new()))
            } else {
                ::std::result::Result::Err(::tredlib::ParseErr{at: $pos})
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_or {
    ($pos:ident, $text:ident, $out:expr, $x1:expr $(, $x2:expr)*) => {
        {
            if let ::std::result::Result::Ok(__res) = $x1 { 
                _tredgen_append!($pos, $out, __res);
            } 
            $(
                else if let ::std::result::Result::Ok(__res) = $x2 {
                   _tredgen_append!($pos, $out, __res);
                }
            )*
            else { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos}); }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_not {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        if let ::std::result::Result::Ok(_) = $x { 
           return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos});
        }
    };
}

#[macro_export]
macro_rules! _tredgen_option {
    ($pos:ident, $text:ident, $out:expr, $x1:expr $(, $x2:expr)*) => {
        if let ::std::result::Result::Ok(mut __res) = $x1 {
            _tredgen_append!($pos, $out, __res);
        }
        $(else if let ::std::result::Result::Ok(mut __res) = $x2 {
            _tredgen_append!($pos, $out, __res);
        })*
    };
}

#[macro_export]
macro_rules! _tredgen_many {
    ($pos:ident, $text:ident, $out:expr, $x1:expr $(, $x2:expr)*) => {
        {
            let mut __mark = false;
            loop {
                if let ::std::result::Result::Ok(mut __res) = $x1 {
                    _tredgen_append!($pos, $out, __res);
                } else {
                    if !__mark { return ::std::result::Result::Err(::tredlib::ParseErr{at: $pos}); }
                    else { break; }
                }
                __mark = true;
                $(if let ::std::result::Result::Ok(mut __res) = $x2 {
                    _tredgen_append!($pos, $out, __res);
                } else {
                    break;
                })*
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_some {
    ($pos:ident, $text:ident, $out:expr $(, $x:expr)+) => {
        loop {
            $(if let ::std::result::Result::Ok(mut __res) = $x {
                _tredgen_append!($pos, $out, __res);
            } else {
                break; 
            })+
        }
    };
}

#[macro_export]
macro_rules! _tredgen_all {
    ($pos:ident, $text:ident, $out:expr $(, $x:expr)+) => {
        while $pos < $text.len() {
            $(match $x {
                Ok(mut __res) => _tredgen_append!($pos, $out, __res),
                __e @ _ => return __e,
            })+
        }
    };
}