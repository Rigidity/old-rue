use std::ops::Range;

use crate::TokenKind;

pub struct Token<'a> {
    pub text: &'a str,
    pub kind: TokenKind,
    pub span: Range<u32>,
}
