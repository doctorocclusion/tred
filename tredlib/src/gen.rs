#[macro_export]
macro_rules! _gen_append {
    ($pos:ident, $text:ident, $out:ident, $res:ident) => {
        $text = $res.0;
        $pos = $res.1;
        $out.append(&mut $res.2);
    }
}


#[macro_export]
macro_rules! _gen_match_str {
    ($pos:ident, $text:ident, $out:ident, $x:expr) => {
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
macro_rules! _gen_match_regex {
    ($pos:ident, $text:ident, $out:ident, $x:expr) => {
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
macro_rules! _gen_capture {
    ($pos:ident, $text:ident, $orig:ident, $out:ident, $x:expr) => {
        {
            let save = $pos;
            $x;
            String::from($orig[save..$pos])
        }
    }
}

#[macro_export]
macro_rules! _gen_or {
    ($pos:ident, $text:ident, $out:ident, $x1:expr, $($x2:expr),*) => {
        {
            if let Ok(mut res) = $x1 { 
                _gen_append!($pos, $text, $out, res);
            } 
            $(
                else if let Ok(mut res) = $x2 {
                    _gen_append!($pos, $text, $out, res);
                }
            )*
            else { return Err(ParseErr{at: $pos}); }
        }
    };
}

#[macro_export]
macro_rules! _gen_many {
    ($pos:ident, $text:ident, $out:ident, $x:expr) => {
        loop {
            if let Ok(mut res) = $x {
                _gen_append!($pos, $text, $out, res);
            } else {
                break; 
            }
        }
    };
}