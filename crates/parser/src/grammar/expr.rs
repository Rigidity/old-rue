use lexer::TokenKind;

use crate::Parser;

pub fn expr(p: &mut Parser) {
    match p.peek() {
        Some(TokenKind::Ident | TokenKind::String) => p.bump(),
        _ => {}
    }
}
