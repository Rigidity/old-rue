use syntax::SyntaxKind;

#[derive(Default)]
pub struct Input {
    kinds: Vec<SyntaxKind>,
    joint: Vec<bool>,
}

impl Input {
    pub fn push(&mut self, kind: SyntaxKind) {
        if self.joint.len() < self.kinds.len() {
            self.joint.push(false);
        }

        self.kinds.push(kind);
    }

    pub fn was_joint(&mut self) {
        if self.kinds.len() < self.joint.len() {
            return;
        }

        self.joint.push(true);
    }

    pub fn kind(&self, index: usize) -> SyntaxKind {
        self.kinds.get(index).copied().unwrap_or(SyntaxKind::Eof)
    }

    pub fn is_joint(&self, index: usize) -> bool {
        self.joint.get(index).copied().unwrap_or(false)
    }
}
