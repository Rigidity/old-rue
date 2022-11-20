use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToPrimitive, FromPrimitive)]
pub enum SyntaxKind {
    Ident,
    DefKw,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Star,
    Slash,
    Arrow,
    Colon,
    Comma,
    Dot,
    Whitespace,
    Error,

    Root,
}

impl From<TokenKind> for SyntaxKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Ident => Self::Ident,
            TokenKind::DefKw => Self::DefKw,
            TokenKind::OpenParen => Self::OpenParen,
            TokenKind::CloseParen => Self::CloseParen,
            TokenKind::OpenBrace => Self::OpenBrace,
            TokenKind::CloseBrace => Self::CloseBrace,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Arrow => Self::Arrow,
            TokenKind::Colon => Self::Colon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Dot => Self::Dot,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Error => Self::Error,
        }
    }
}
