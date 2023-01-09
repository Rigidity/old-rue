use syntax::{Set, SyntaxKind, EMPTY_SET, T};

use crate::Parser;

use super::{stmts, types};

const PARAM_START: Set = Set::new(&[SyntaxKind::Ident]).union(types::TYPE_START);

pub fn item(p: &mut Parser) {
    match p.peek() {
        T![def] => def_item(p),
        _ => {
            p.err_recover("Expected an item", EMPTY_SET);
        }
    }
}

fn def_item(p: &mut Parser) {
    let m = p.start();

    p.bump(T![def]);
    p.expect(SyntaxKind::Ident);
    param_list(p);

    if p.eat(T![->]) {
        types::type_(p);
    }

    block(p);

    m.complete(p, SyntaxKind::DefItem);
}

fn param_list(p: &mut Parser) {
    let m = p.start();

    p.expect(T!['(']);

    while !p.at(T![')']) && !p.at(SyntaxKind::Eof) {
        if !p.at_set(PARAM_START) {
            p.error("Expected parameter");
            m.abandon(p);
            return;
        }

        param(p);

        if !p.at(T![')']) {
            p.expect(T![,]);
        }
    }

    p.expect(T![')']);

    m.complete(p, SyntaxKind::ParamList);
}

fn param(p: &mut Parser) {
    let m = p.start();

    p.expect(SyntaxKind::Ident);
    p.expect(T![:]);
    types::type_(p);

    m.complete(p, SyntaxKind::Param);
}

fn block(p: &mut Parser) {
    let m = p.start();

    p.expect(T!['{']);

    while !p.at(T!['}']) && !p.at(SyntaxKind::Eof) {
        stmts::stmt(p);
    }

    p.expect(T!['}']);

    m.complete(p, SyntaxKind::Block);
}
