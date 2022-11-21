use crate::SyntaxKind;

pub const EMPTY_SET: Set = Set::new(&[]);

#[derive(Clone, Copy)]
pub struct Set(u128);

impl Set {
    pub const EMPTY: Set = Set(0);

    pub const fn new(kinds: &[SyntaxKind]) -> Set {
        let mut res = 0u128;
        let mut i = 0;

        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1;
        }

        Set(res)
    }

    pub const fn union(self, other: Set) -> Set {
        Set(self.0 | other.0)
    }

    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & mask(kind) != 0
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_set_works_for_tokens() {
        let ts = Set::new(&[SyntaxKind::Eof, SyntaxKind::Colon]);
        assert!(ts.contains(SyntaxKind::Eof));
        assert!(ts.contains(SyntaxKind::Colon));
        assert!(!ts.contains(SyntaxKind::Plus));
    }
}
