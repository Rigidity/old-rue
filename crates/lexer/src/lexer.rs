use std::{iter::Peekable, str::Chars};

use crate::{
    base::Base,
    chars::{is_ident_continue, is_ident_start, is_whitespace},
    Token, TokenKind,
};

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
            '/' => match self.char() {
                '/' => {
                    self.bump();
                    self.line_comment()
                }
                '*' => {
                    self.bump();
                    self.block_comment()
                }
                _ => TokenKind::Slash,
            },
            '%' => TokenKind::Percent,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '!' => TokenKind::Exclamation,
            '=' => TokenKind::Equals,
            '"' => self.string('"'),
            '\'' => self.string('\''),
            c @ '0'..='9' => self.integer(c),
            c if is_ident_start(c) => self.ident(start),
            c if is_whitespace(c) => self.whitespace(),
            _ => TokenKind::Error,
        };

        Token {
            text: &self.source[start..self.cursor],
            kind,
            span: start..self.cursor,
        }
    }

    fn string(&mut self, quote: char) -> TokenKind {
        let is_terminated = loop {
            match self.bump() {
                '\0' => break false,
                c if c == quote => break true,
                _ => {}
            }
        };
        TokenKind::String { is_terminated }
    }

    fn integer(&mut self, digit: char) -> TokenKind {
        let mut base = Base::Decimal;

        let has_digits = if digit == '0' {
            match self.char() {
                'b' => {
                    base = Base::Binary;
                    self.bump();
                    self.eat_decimal_digits()
                }
                'o' => {
                    base = Base::Octal;
                    self.bump();
                    self.eat_decimal_digits()
                }
                'x' => {
                    base = Base::Hexadecimal;
                    self.bump();
                    self.eat_hexadecimal_digits()
                }
                _ => {
                    self.eat_decimal_digits();
                    true
                }
            }
        } else {
            self.eat_decimal_digits();
            true
        };

        TokenKind::Integer {
            base,
            is_empty: !has_digits,
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.char() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.char() {
                '_' => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn ident(&mut self, start: usize) -> TokenKind {
        while is_ident_continue(self.char()) {
            self.bump();
        }

        match &self.source[start..self.cursor] {
            "def" => TokenKind::DefKw,
            "let" => TokenKind::LetKw,
            "true" => TokenKind::TrueKw,
            "false" => TokenKind::FalseKw,
            _ => TokenKind::Ident,
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        while is_whitespace(self.char()) {
            self.bump();
        }
        TokenKind::Whitespace
    }

    fn line_comment(&mut self) -> TokenKind {
        loop {
            match self.char() {
                '\n' | '\0' => break,
                _ => {
                    self.bump();
                }
            }
        }
        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        let is_terminated = loop {
            match self.char() {
                '\0' => break false,
                '*' => {
                    self.bump();
                    if self.char() == '/' {
                        self.bump();
                        break true;
                    }
                }
                _ => {
                    self.bump();
                }
            }
        };
        TokenKind::BlockComment { is_terminated }
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
    fn string() {
        assert_eq!(
            lex("'abc'"),
            &[TokenKind::String {
                is_terminated: true
            }]
        )
    }

    #[test]
    fn integer() {
        assert_eq!(
            lex("42"),
            &[TokenKind::Integer {
                base: Base::Decimal,
                is_empty: false
            }]
        )
    }

    #[test]
    fn def_kw() {
        assert_eq!(lex("def"), &[TokenKind::DefKw])
    }

    #[test]
    fn let_kw() {
        assert_eq!(lex("let"), &[TokenKind::LetKw])
    }

    #[test]
    fn true_kw() {
        assert_eq!(lex("true"), &[TokenKind::TrueKw])
    }

    #[test]
    fn false_kw() {
        assert_eq!(lex("false"), &[TokenKind::FalseKw])
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

    #[test]
    fn line_comment() {
        assert_eq!(lex("// comment"), &[TokenKind::LineComment])
    }

    #[test]
    fn block_comment() {
        assert_eq!(
            lex("/* comment */"),
            &[TokenKind::BlockComment {
                is_terminated: true
            }]
        )
    }
}
