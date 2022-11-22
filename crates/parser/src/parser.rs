use lexer::Token;
use syntax::{Set, SyntaxKind, T};

use crate::{event::Event, grammar::root, input::Input, output::Output, sink::Sink};

mod completed_marker;
mod marker;

pub(crate) use completed_marker::CompletedMarker;
pub(crate) use marker::Marker;

pub struct Parser {
    input: Input,
    cursor: usize,
    events: Vec<Event>,
}

impl Parser {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            cursor: 0,
            events: Vec::new(),
        }
    }

    pub fn parse_tokens(tokens: &[Token]) -> Output {
        let mut parser = Self::new(Input::from_tokens(tokens));
        root(&mut parser);
        parser.parse(tokens)
    }

    pub(crate) fn parse(self, tokens: &[Token]) -> Output {
        Sink::new(self.finish(), tokens).finish()
    }

    fn finish(self) -> Vec<Event> {
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::tombstone());
        Marker::new(pos)
    }

    pub(crate) fn peek(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        self.input.kind(self.cursor + n)
    }

    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        match kind {
            T![->] => self.at_composite2(n, T![-], T![>]),
            T![<=] => self.at_composite2(n, T![<], T![=]),
            T![>=] => self.at_composite2(n, T![>], T![=]),
            T![==] => self.at_composite2(n, T![=], T![=]),
            T![!=] => self.at_composite2(n, T![!], T![=]),
            T![<<] => self.at_composite2(n, T![<], T![<]),
            T![>>] => self.at_composite2(n, T![>], T![>]),
            T![>>>] => self.at_composite3(n, T![>], T![>], T![>]),
            _ => self.input.kind(self.cursor + n) == kind,
        }
    }

    pub(crate) fn at_set(&self, kinds: Set) -> bool {
        kinds.contains(self.peek())
    }

    fn at_composite2(&self, n: usize, a: SyntaxKind, b: SyntaxKind) -> bool {
        self.nth(n) == a && self.nth(n + 1) == b && self.input.is_joint(n)
    }

    fn at_composite3(&self, n: usize, a: SyntaxKind, b: SyntaxKind, c: SyntaxKind) -> bool {
        self.nth(n) == a
            && self.nth(n + 1) == b
            && self.nth(n + 2) == c
            && self.input.is_joint(n)
            && self.input.is_joint(n + 1)
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }

        let token_count = match kind {
            T![->] => 2,
            T![<=] => 2,
            T![>=] => 2,
            T![==] => 2,
            T![!=] => 2,
            T![<<] => 2,
            T![>>] => 2,
            T![>>>] => 3,
            _ => 1,
        };

        self.add_token(kind, token_count);
        true
    }

    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == SyntaxKind::Eof {
            return;
        }
        self.add_token(kind, 1);
    }

    pub(crate) fn error<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.events.push(Event::Error(message.into()));
    }

    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
        false
    }

    pub(crate) fn err_recover(&mut self, message: &str, recovery: Set) {
        match self.peek() {
            T!['{'] | T!['}'] => {
                self.error(message);
                return;
            }
            _ => (),
        }

        if self.at_set(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, SyntaxKind::Error);
    }

    pub(crate) fn add_token(&mut self, kind: SyntaxKind, token_count: usize) {
        self.cursor += token_count;
        self.events.push(Event::AddToken { kind, token_count })
    }
}
