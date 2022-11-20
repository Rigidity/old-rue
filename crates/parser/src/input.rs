use lexer::{Token, TokenKind};

pub struct Input<'a, 't> {
    source: &'a [Token<'t>],
    cursor: usize,
}

impl<'a, 't> Input<'a, 't> {
    pub fn new(source: &'a [Token<'t>]) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<&'a Token<'t>> {
        self.eat_whitespace();
        let lexeme = self.source.get(self.cursor)?;
        self.cursor += 1;
        Some(lexeme)
    }

    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.eat_whitespace();
        self.peek_kind_raw()
    }

    fn eat_whitespace(&mut self) {
        while self.peek_kind_raw().map_or(false, |kind| kind.is_trivia()) {
            self.cursor += 1;
        }
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
        self.source.get(self.cursor).map(|token| token.kind)
    }
}
