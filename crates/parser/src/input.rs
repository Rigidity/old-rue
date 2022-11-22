use lexer::Token;
use syntax::SyntaxKind;

#[derive(Default)]
pub struct Input {
    kinds: Vec<SyntaxKind>,
    joint: Vec<bool>,
}

impl Input {
    pub fn from_tokens(tokens: &[Token]) -> Self {
        let mut result = Self::default();
        let mut joint = false;

        for token in tokens {
            if token.kind.is_trivia() {
                joint = false;
            } else {
                if joint {
                    result.was_joint();
                } else {
                    joint = true;
                }

                result.push(token.kind.into());
            }
        }

        result
    }

    pub fn push(&mut self, kind: SyntaxKind) {
        self.kinds.push(kind);
        self.joint.push(false);
    }

    pub fn was_joint(&mut self) {
        if let Some(last) = self.joint.last_mut() {
            *last = true;
        }
    }

    pub fn kind(&self, index: usize) -> SyntaxKind {
        self.kinds.get(index).copied().unwrap_or(SyntaxKind::Eof)
    }

    pub fn is_joint(&self, index: usize) -> bool {
        self.joint.get(index).copied().unwrap_or(false)
    }
}
