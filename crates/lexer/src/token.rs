use std::ops::Range;

use crate::TokenKind;

#[derive(Debug)]
pub struct Token<'a> {
    pub text: &'a str,
    pub kind: TokenKind,
    pub span: Range<usize>,
}
