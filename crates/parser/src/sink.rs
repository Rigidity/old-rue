use lexer::Token;
use rowan::{GreenNodeBuilder, Language};
use syntax::{RueLanguage, SyntaxKind};

use crate::{event::Event, output::Output};

pub struct Sink<'a, 't> {
    events: Vec<Event<'a>>,
    source: &'a [Token<'t>],
    cursor: usize,
    builder: GreenNodeBuilder<'static>,
}

impl<'a, 't> Sink<'a, 't> {
    pub fn new(events: Vec<Event<'a>>, source: &'a [Token<'t>]) -> Self {
        Self {
            events,
            source,
            cursor: 0,
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn finish(mut self) -> Output {
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(RueLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
            }

            self.eat_trivia();
        }

        Output {
            green_node: self.builder.finish(),
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.source.get(self.cursor) {
            if token.kind.is_trivia() {
                self.token(token.kind.into(), token.text);
            }
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: &'a str) {
        self.builder.token(RueLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }
}
