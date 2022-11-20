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
    Placeholder,
}
