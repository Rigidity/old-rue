#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    Ident,
    Whitespace,
    Error,
}
