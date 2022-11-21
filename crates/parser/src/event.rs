use syntax::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        token_count: usize,
    },
    Error(String),
    FinishNode,
}

impl Event {
    pub fn tombstone() -> Self {
        Self::StartNode {
            kind: SyntaxKind::Tombstone,
            forward_parent: None,
        }
    }
}
