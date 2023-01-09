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
    Percent,
    And,
    Or,
    Xor,
    Arrow,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Equals,
    Exclamation,
    Whitespace,
    LineComment,
    BlockComment,
    Error,

    Eof,
    Tombstone,

    LazyAnd,
    LazyOr,
    EqualTo,
    NotEqual,
    LessThanEquals,
    GreaterThanEquals,
    LeftShift,
    RightShift,
    UnsignedRightShift,

    Root,

    Literal,
    NameRef,
    ParenExpr,
    BinaryExpr,
    PrefixExpr,

    DefItem,
    ParamList,
    Param,
    Block,

    LetStmt,

    NameType,
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
            TokenKind::Percent => Self::Percent,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            TokenKind::Xor => Self::Xor,
            TokenKind::Arrow => Self::Arrow,
            TokenKind::Colon => Self::Colon,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Dot => Self::Dot,
            TokenKind::Equals => Self::Equals,
            TokenKind::Exclamation => Self::Exclamation,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::LineComment => Self::LineComment,
            TokenKind::BlockComment { .. } => Self::BlockComment,
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
    [%] => { SyntaxKind::Percent };
    [&] => { SyntaxKind::And };
    [|] => { SyntaxKind::Or };
    [^] => { SyntaxKind::Xor };
    [->] => { SyntaxKind::Arrow };
    [:] => { SyntaxKind::Colon };
    [;] => { SyntaxKind::Semicolon };
    [,] => { SyntaxKind::Comma };
    [.] => { SyntaxKind::Dot };
    [=] => { SyntaxKind::Equals };
    [!] => { SyntaxKind::Exclamation };
    [&&] => { SyntaxKind::LazyAnd };
    [||] => { SyntaxKind::LazyOr };
    [==] => { SyntaxKind::EqualTo };
    [!=] => { SyntaxKind::NotEqual };
    [<=] => { SyntaxKind::LessThanEquals };
    [>=] => { SyntaxKind::GreaterThanEquals };
    [<<] => { SyntaxKind::LeftShift };
    [>>] => { SyntaxKind::RightShift };
    [>>>] => { SyntaxKind::UnsignedRightShift };
}

pub use T;
