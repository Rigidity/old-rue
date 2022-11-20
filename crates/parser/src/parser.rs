use lexer::{Token, TokenKind};

use crate::{event::Event, grammar::root, input::Input, output::Output, sink::Sink};

use self::marker::Marker;

pub mod marker;

pub struct Parser {
    input: Input,
    cursor: usize,
    events: Vec<Event>,
}

impl Parser {
    fn new(input: Input) -> Self {
        Self {
            input,
            cursor: 0,
            events: Vec::new(),
        }
    }

    pub fn parse(source: &[Token]) -> Output {
        let mut input = Input::default();
        let mut joint = false;

        for token in source {
            if token.kind.is_trivia() {
                joint = false;
            } else {
                input.push(token.kind);

                if joint {
                    input.was_joint();
                }
            }
        }

        let mut parser = Self::new(input);
        root(&mut parser);
        Sink::new(parser.finish(), source).finish()
    }

    fn finish(self) -> Vec<Event> {
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub(crate) fn peek(&mut self) -> Option<TokenKind> {
        self.nth(0)
    }

    pub(crate) fn nth(&mut self, index: usize) -> Option<TokenKind> {
        self.input.kind(self.cursor + index)
    }

    pub(crate) fn bump(&mut self) {
        let kind = self.peek().unwrap();
        self.cursor += 1;

        self.events.push(Event::AddToken {
            kind: kind.into(),
            token_count: 1,
        })
    }
}
