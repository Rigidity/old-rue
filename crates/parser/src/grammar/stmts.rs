use syntax::{SyntaxKind, EMPTY_SET, T};

use crate::Parser;

use super::{exprs, types};

pub fn stmt(p: &mut Parser) {
    match p.peek() {
        T![let] => let_stmt(p),
        _ => {
            p.err_recover("Expected a statement", EMPTY_SET);
        }
    }
}

fn let_stmt(p: &mut Parser) {
    let m = p.start();

    p.bump(T![let]);

    p.expect(SyntaxKind::Ident);

    if p.eat(T![:]) {
        types::type_(p);
    }

    if p.eat(T![=]) {
        exprs::expr(p);
    }

    p.expect(T![;]);

    m.complete(p, SyntaxKind::LetStmt);
}
