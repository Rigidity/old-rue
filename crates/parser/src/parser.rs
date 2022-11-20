use lexer::{Token, TokenKind};

use crate::{event::Event, grammar::root, output::Output, sink::Sink};

use self::marker::Marker;

pub mod marker;

pub struct Parser<'a, 't> {
    source: &'a [Token<'t>],
    cursor: usize,
    events: Vec<Event<'a>>,
}

impl<'a, 't> Parser<'a, 't> {
    pub fn new(source: &'a [Token<'t>]) -> Self {
        Self {
            source,
            cursor: 0,
            events: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Output {
        root(&mut self);
        Sink::new(self.events, self.source).finish()
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub(crate) fn bump(&mut self) {
        let token = self.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: token.kind.into(),
            text: token.text,
        })
    }

    pub(crate) fn peek(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.peek_raw()
    }

    fn next_token(&mut self) -> Option<&'a Token<'t>> {
        self.eat_trivia();
        let lexeme = self.source.get(self.cursor)?;
        self.cursor += 1;
        Some(lexeme)
    }

    fn eat_trivia(&mut self) {
        while self.peek_raw().map_or(false, |kind| kind.is_trivia()) {
            self.cursor += 1;
        }
    }

    fn peek_raw(&self) -> Option<TokenKind> {
        self.source.get(self.cursor).map(|token| token.kind)
    }
}
