use unicode_xid::UnicodeXID;

pub fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_xid_start()
}

pub fn is_ident_continue(c: char) -> bool {
    c.is_xid_continue()
}

/// From [Rust lexer](https://github.com/rust-lang/rust/blob/ff0ffda6b3c3ea392c6cf88c676133666f491e5c/compiler/rustc_lexer/src/lib.rs#L271).
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        | '\u{0009}' // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}
