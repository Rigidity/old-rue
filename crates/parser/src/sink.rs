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
        for i in 0..self.events.len() {
            match std::mem::replace(&mut self.events[i], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];

                    let mut i = i;
                    let mut forward_parent = forward_parent;

                    while let Some(fp) = forward_parent {
                        i += fp;

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            std::mem::replace(&mut self.events[i], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(RueLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
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
            } else {
                break;
            }
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: &'a str) {
        self.builder.token(RueLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }
}
