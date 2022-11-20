#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    Ident,
    String,
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
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}
