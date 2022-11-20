use syntax::SyntaxKind;

#[derive(Debug, Clone)]
pub enum Event<'a> {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: &'a str },
    FinishNode,
}
