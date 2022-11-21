use lexer::Token;
use rowan::{GreenNodeBuilder, Language, TextRange, TextSize};
use syntax::{RueLanguage, SyntaxKind};

use crate::{event::Event, output::Output, parse_error::ParseError};

pub struct Sink<'a, 't> {
    events: Vec<Event>,
    tokens: &'a [Token<'t>],
    cursor: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<ParseError>,
}

impl<'a, 't> Sink<'a, 't> {
    pub fn new(events: Vec<Event>, tokens: &'a [Token<'t>]) -> Self {
        Self {
            events,
            tokens,
            cursor: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    pub fn finish(mut self) -> Output {
        let mut forward_parents = Vec::new();

        for i in 0..self.events.len() {
            match std::mem::replace(&mut self.events[i], Event::tombstone()) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    forward_parents.push(kind);
                    let mut idx = i;
                    let mut fp = forward_parent;
                    while let Some(fwd) = fp {
                        idx += fwd as usize;
                        fp = match std::mem::replace(&mut self.events[idx], Event::tombstone()) {
                            Event::StartNode {
                                kind,
                                forward_parent,
                            } => {
                                forward_parents.push(kind);
                                forward_parent
                            }
                            _ => unreachable!(),
                        };
                    }

                    for kind in forward_parents.drain(..).rev() {
                        if kind != SyntaxKind::Tombstone {
                            self.builder.start_node(RueLanguage::kind_to_raw(kind));
                        }
                    }
                }
                Event::FinishNode => self.builder.finish_node(),
                Event::AddToken { kind, token_count } => self.token(kind, token_count),
                Event::Error(message) => {
                    let span = match self.tokens.get(self.cursor) {
                        Some(token) => TextRange::new(
                            TextSize::from(token.span.start as u32),
                            TextSize::from(token.span.end as u32),
                        ),
                        None => TextRange::default(),
                    };
                    self.errors.push(ParseError { message, span });
                }
            }

            self.eat_trivia();
        }

        Output {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if token.kind.is_trivia() {
                self.token(token.kind.into(), 1);
            } else {
                break;
            }
        }
    }

    fn token(&mut self, kind: SyntaxKind, token_count: usize) {
        let mut text = String::new();
        let tokens = &self.tokens[self.cursor..self.cursor + token_count];

        for token in tokens {
            text.push_str(token.text);
        }

        self.builder.token(RueLanguage::kind_to_raw(kind), &text);
        self.cursor += token_count;
    }
}
