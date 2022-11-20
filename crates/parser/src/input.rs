use lexer::TokenKind;

#[derive(Default)]
pub struct Input {
    kinds: Vec<TokenKind>,
    joint: Vec<bool>,
}

impl Input {
    pub fn push(&mut self, kind: TokenKind) {
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

    pub fn kind(&self, index: usize) -> Option<TokenKind> {
        self.kinds.get(index).copied()
    }

    pub fn is_joint(&self, index: usize) -> bool {
        self.joint.get(index).copied().unwrap_or(false)
    }
}
