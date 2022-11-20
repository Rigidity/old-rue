use lexer::{Token, TokenKind};
use syntax::SyntaxKind;

use crate::{event::Event, input::Input, output::Output, sink::Sink};

pub struct Parser<'a, 't> {
    input: Input<'a, 't>,
    events: Vec<Event<'a>>,
}

impl<'a, 't> Parser<'a, 't> {
    pub fn new(source: &'a [Token<'t>]) -> Self {
        Self {
            input: Input::new(source),
            events: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Output {
        self.start_node(SyntaxKind::Root);
        self.finish_node();
        Sink::new(self.events, self.source).finish()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.input.peek_kind()
    }

    fn bump(&mut self) {
        let token = self.input.next_token().unwrap();

        self.events.push(Event::AddToken {
            kind: token.kind.into(),
            text: token.text,
        })
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }
}
