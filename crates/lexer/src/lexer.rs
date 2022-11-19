use std::{iter::Peekable, str::Chars};

use crate::{Token, TokenKind};

use self::chars::{is_ident_continue, is_ident_start, is_whitespace};

mod chars;

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

    fn token(&mut self) -> Token<'a> {
        let start = self.cursor;

        let kind = match self.bump() {
            c if is_ident_start(c) => {
                self.eat_ident();
                TokenKind::Ident
            }
            c if is_whitespace(c) => {
                self.eat_whitespace();
                TokenKind::Whitespace
            }
            _ => TokenKind::Error,
        };

        Token {
            text: &self.source[start..self.cursor],
            kind,
            span: start..self.cursor,
        }
    }

    fn eat_ident(&mut self) {
        while is_ident_continue(self.char()) {
            self.bump();
        }
    }

    fn eat_whitespace(&mut self) {
        while is_whitespace(self.char()) {
            self.bump();
        }
    }

    fn char(&mut self) -> char {
        *self.chars.peek().unwrap_or(&'\0')
    }

    fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(c) => {
                self.cursor += c.len_utf8();
                c
            }
            None => '\0',
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
