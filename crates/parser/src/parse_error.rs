use rowan::TextRange;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: TextRange,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}..{}",
            self.message,
            u32::from(self.span.start()),
            u32::from(self.span.end()),
        )
    }
}
