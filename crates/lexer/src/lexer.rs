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
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '+' => TokenKind::Plus,
            '-' => match self.char() {
                '>' => {
                    self.bump();
                    TokenKind::Arrow
                }
                _ => TokenKind::Minus,
            },
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            c if is_ident_start(c) => {
                self.eat_ident();
                match &self.source[start..self.cursor] {
                    "def" => TokenKind::DefKw,
                    _ => TokenKind::Ident,
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<TokenKind> {
        Lexer::new(source)
            .map(|token| token.kind)
            .collect::<Vec<_>>()
    }

    #[test]
    fn ident() {
        assert_eq!(lex("hello_world"), &[TokenKind::Ident])
    }

    #[test]
    fn def_kw() {
        assert_eq!(lex("def"), &[TokenKind::DefKw])
    }

    #[test]
    fn open_paren() {
        assert_eq!(lex("("), &[TokenKind::OpenParen])
    }

    #[test]
    fn close_paren() {
        assert_eq!(lex(")"), &[TokenKind::CloseParen])
    }

    #[test]
    fn open_brace() {
        assert_eq!(lex("{"), &[TokenKind::OpenBrace])
    }

    #[test]
    fn close_brace() {
        assert_eq!(lex("}"), &[TokenKind::CloseBrace])
    }

    #[test]
    fn less_than() {
        assert_eq!(lex("<"), &[TokenKind::LessThan])
    }

    #[test]
    fn greater_than() {
        assert_eq!(lex(">"), &[TokenKind::GreaterThan])
    }

    #[test]
    fn plus() {
        assert_eq!(lex("+"), &[TokenKind::Plus])
    }

    #[test]
    fn minus() {
        assert_eq!(lex("-"), &[TokenKind::Minus])
    }

    #[test]
    fn star() {
        assert_eq!(lex("*"), &[TokenKind::Star])
    }

    #[test]
    fn slash() {
        assert_eq!(lex("/"), &[TokenKind::Slash])
    }

    #[test]
    fn arrow() {
        assert_eq!(lex("->"), &[TokenKind::Arrow])
    }

    #[test]
    fn colon() {
        assert_eq!(lex(":"), &[TokenKind::Colon])
    }

    #[test]
    fn comma() {
        assert_eq!(lex(","), &[TokenKind::Comma])
    }

    #[test]
    fn dot() {
        assert_eq!(lex("."), &[TokenKind::Dot])
    }

    #[test]
    fn whitespace() {
        assert_eq!(lex("    "), &[TokenKind::Whitespace])
    }
}
