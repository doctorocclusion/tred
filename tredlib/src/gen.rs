#[macro_export]
macro_rules! _tredgen_append {
    ($pos:ident, $text:ident, $out:expr, $res:ident) => {
        $text = $res.0;
        $pos = $res.1;
        $out(&mut $res.2);
    }
}


#[macro_export]
macro_rules! _tredgen_match_str {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        {
            let tmp = $x;
            let len = tmp.len();
            if $text.starts_with(tmp) {
                $text = &$text[len..]; 
                $pos += len;
            } else {
                return Err(ParseErr{at: $pos});
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_match_regex {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        {
            if let Some((_, end)) = $x.find($text) {
                $pos += end;
                $text = &$text[end..];
            } else {
                return Err(ParseErr{at: $pos});
            }
        }
    }
}

#[macro_export]
macro_rules! _tredgen_capture {
    ($pos:ident, $text:ident, $orig:ident, $out:expr, $x:expr) => {
        {
            let save = $pos;
            $x;
            String::from($orig[save..$pos])
        }
    }
}

#[macro_export]
macro_rules! _tredgen_or {
    ($pos:ident, $text:ident, $out:expr, $x1:expr, $($x2:expr),*) => {
        {
            if let Ok(mut res) = $x1 { 
                _tredgen_append!($pos, $text, $out, res);
            } 
            $(
                else if let Ok(mut res) = $x2 {
                    _tredgen_append!($pos, $text, $out, res);
                }
            )*
            else { return Err(ParseErr{at: $pos}); }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_many {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        {
            let mut mark = false;
            loop {
                if let Ok(mut res) = $x {
                    mark = true;
                    _tredgen_append!($pos, $text, $out, res);
                } else {
                    if !mark { return Err(ParseErr{at: $pos}); }
                    else { break; }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_some {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        loop {
            if let Ok(mut res) = $x {
                _tredgen_append!($pos, $text, $out, res);
            } else {
                break; 
            }
        }
    };
}

#[macro_export]
macro_rules! _tredgen_all {
    ($pos:ident, $text:ident, $out:expr, $x:expr) => {
        while $text.len() > 0 {
            match $x {
                Ok(mut res) => _tredgen_append!($pos, $text, $out, res),
                e @ _ => return e;
            }
        }
    };
}