use drop_bomb::DropBomb;
use syntax::SyntaxKind;

use crate::{event::Event, Parser};

pub struct Marker {
    pos: usize,
    bomb: DropBomb,
}

pub struct CompletedMarker {
    pos: usize,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        p.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

impl CompletedMarker {
    pub fn precede(self, p: &mut Parser) -> Marker {
        let m = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.pos]
        {
            *forward_parent = Some(m.pos - self.pos);
        } else {
            unreachable!();
        }

        m
    }
}
