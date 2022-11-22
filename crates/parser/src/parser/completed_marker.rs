use crate::{event::Event, Parser};

use super::Marker;

pub(crate) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub fn new(pos: usize) -> Self {
        Self { pos }
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
}
