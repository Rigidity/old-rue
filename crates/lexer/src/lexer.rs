use std::{iter::Peekable, str::Chars};

use crate::{Token, TokenKind};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            cursor: 0,
        }
    }

    fn has_token(&self) -> bool {
        self.cursor < self.source.len()
    }

    fn token(&self) -> Token<'a> {
        Token {
            text: "",
            kind: TokenKind::Error,
            span: 0..0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = if self.has_token() {
            self.token()
        } else {
            return None;
        };

        Some(token)
    }
}
