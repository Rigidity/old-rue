use syntax::{Set, SyntaxKind, T};

use crate::Parser;

pub const TYPE_START: Set = Set::new(&[SyntaxKind::Ident]);

const TYPE_RECOVERY_SET: Set = Set::new(&[T![')'], T![,]]);

pub(super) fn type_(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => {
            let m = p.start();
            p.bump_any();
            m.complete(p, SyntaxKind::NameType);
        }
        _ => {
            p.err_recover("Expected type", TYPE_RECOVERY_SET);
        }
    }
}
