use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToPrimitive, FromPrimitive)]
pub enum SyntaxKind {
    Ident,
    String,
    Integer,
    DefKw,
    LetKw,
    TrueKw,
    FalseKw,
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

    Eof,
    Tombstone,

    Root,
    Literal,
    NameRef,
    ParenExpr,
    BinaryExpr,
    PrefixExpr,
}

impl From<TokenKind> for SyntaxKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Ident { .. } => Self::Ident,
            TokenKind::String { .. } => Self::String,
            TokenKind::Integer { .. } => Self::Integer,
            TokenKind::DefKw => Self::DefKw,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::TrueKw => Self::TrueKw,
            TokenKind::FalseKw => Self::FalseKw,
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

#[macro_export]
macro_rules ! T {
    [def] => { SyntaxKind::DefKw };
    [let] => { SyntaxKind::LetKw };
    [true] => { SyntaxKind::TrueKw };
    [false] => { SyntaxKind::FalseKw };
    ['('] => { SyntaxKind::OpenParen };
    [')'] => { SyntaxKind::CloseParen };
    ['{'] => { SyntaxKind::OpenBrace };
    ['}'] => { SyntaxKind::CloseBrace };
    [<] => { SyntaxKind::LessThan };
    [>] => { SyntaxKind::GreaterThan };
    [+] => { SyntaxKind::Plus };
    [-] => { SyntaxKind::Minus };
    [*] => { SyntaxKind::Star };
    [/] => { SyntaxKind::Slash };
    [->] => { SyntaxKind::Arrow };
    [:] => { SyntaxKind::Colon };
    [,] => { SyntaxKind::Comma };
    [.] => { SyntaxKind::Dot };
}

pub use T;
