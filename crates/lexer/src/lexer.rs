use std::{iter::Peekable, str::Chars};
use unicode_xid::UnicodeXID;

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

    fn token(&mut self) -> Token<'a> {
        let start = self.cursor;

        let kind = match self.bump() {
            c if c.is_xid_start() => {
                self.eat_ident();
                TokenKind::Ident
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
        while self.char().is_xid_continue() {
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
