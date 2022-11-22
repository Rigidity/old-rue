use drop_bomb::DropBomb;
use syntax::SyntaxKind;

use crate::{event::Event, Parser};

use super::CompletedMarker;

pub(crate) struct Marker {
    pub(super) pos: usize,
    pub(super) bomb: DropBomb,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
        }
    }

    pub fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::StartNode { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        p.events.push(Event::FinishNode);
        CompletedMarker::new(self.pos)
    }

    pub fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::StartNode {
                    kind: SyntaxKind::Tombstone,
                    forward_parent: None,
                }) => {}
                _ => unreachable!(),
            }
        }
    }
}
