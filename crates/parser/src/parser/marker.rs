use drop_bomb::DropBomb;
use syntax::SyntaxKind;

use crate::{event::Event, Parser};

pub struct Marker {
    pos: usize,
    bomb: DropBomb,
}

pub struct CompletedMarker {
    pos: usize,
    kind: SyntaxKind,
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
        CompletedMarker::new(self.pos, kind)
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

impl CompletedMarker {
    pub fn new(pos: usize, kind: SyntaxKind) -> Self {
        Self { pos, kind }
    }

    pub fn precede(self, p: &mut Parser) -> Marker {
        let new_pos = p.start();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::StartNode { forward_parent, .. } => {
                *forward_parent = Some(new_pos.pos - self.pos);
            }
            _ => unreachable!(),
        }
        new_pos
    }

    pub fn extend_to(self, p: &mut Parser, mut m: Marker) -> CompletedMarker {
        m.bomb.defuse();
        let idx = m.pos as usize;
        match &mut p.events[idx] {
            Event::StartNode { forward_parent, .. } => {
                *forward_parent = Some(self.pos - m.pos);
            }
            _ => unreachable!(),
        }
        self
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
