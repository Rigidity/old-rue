use syntax::{Set, SyntaxKind, T};

use crate::{
    parser::{CompletedMarker, Marker},
    Parser,
};

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_bp(p, None, 1)
}

const LITERAL_START: Set =
    Set::new(&[T![true], T![false], SyntaxKind::String, SyntaxKind::Integer]);

fn literal(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at_set(LITERAL_START) {
        return None;
    }

    let m = p.start();
    p.bump_any();
    Some(m.complete(p, SyntaxKind::Literal))
}

#[allow(unused)]
const ATOM_START: Set = LITERAL_START.union(Set::new(&[T!['(']]));

const EXPR_RECOVERY_SET: Set = Set::new(&[T![let]]);

fn atom_expr(p: &mut Parser) -> Option<CompletedMarker> {
    if let Some(cm) = literal(p) {
        return Some(cm);
    }

    let cm = match p.peek() {
        T!['('] => paren_expr(p),
        SyntaxKind::Ident => {
            let m = p.start();
            p.bump_any();
            m.complete(p, SyntaxKind::NameRef)
        }
        _ => {
            p.err_recover("expected expression", EXPR_RECOVERY_SET);
            return None;
        }
    };

    Some(cm)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.expect(T!['(']);
    expr(p);
    p.expect(T![')']);
    m.complete(p, SyntaxKind::ParenExpr)
}

#[rustfmt::skip]
fn current_op(p: &Parser) -> (u8, SyntaxKind) {
    match p.peek() {
        T![|] if p.at(T![||])   => (1, T![||]),
        T![|]                   => (5, T![|]),
        T![^]                   => (6, T![|]),
        T![&] if p.at(T![&&])   => (2, T![&&]),
        T![|]                   => (7, T![|]),
        T![<] if p.at(T![<=])   => (3, T![<=]),
        T![<] if p.at(T![<<])   => (4, T![<<]),
        T![<]                   => (3, T![<]),
        T![>] if p.at(T![>>>])  => (4, T![>>>]),
        T![>] if p.at(T![>>])   => (4, T![>>]),
        T![>] if p.at(T![>=])   => (3, T![>=]),
        T![>]                   => (3, T![>]),
        T![=] if p.at(T![==])   => (3, T![==]),
        T![!] if p.at(T![!=])   => (3, T![!=]),
        T![+]                   => (8, T![+]),
        T![-]                   => (8, T![-]),
        T![*]                   => (9, T![*]),
        T![/]                   => (9, T![/]),
        T![%]                   => (9, T![%]),
        _                       => (0, SyntaxKind::Error),
    }
}

fn expr_bp(p: &mut Parser, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());

    let mut lhs = match lhs(p) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        let (op_bp, op) = current_op(p);

        if op_bp < bp {
            break;
        }

        let m = lhs.precede(p);
        p.bump(op);

        expr_bp(p, None, op_bp + 1);
        lhs = m.complete(p, SyntaxKind::BinaryExpr);
    }

    Some(lhs)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let m;
    let kind = match p.peek() {
        T![-] | T![!] => {
            m = p.start();
            p.bump_any();
            SyntaxKind::PrefixExpr
        }
        _ => return atom_expr(p),
    };

    expr_bp(p, None, 255);
    let cm = m.complete(p, kind);
    Some(cm)
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::grammar::parse;

    use super::*;

    #[test]
    fn parse_true() {
        expect![[r#"Literal@0..4
  TrueKw@0..4 "true""#]]
        .assert_eq(&parse("true", |p| {
            atom_expr(p);
        }));
    }

    #[test]
    fn parse_false() {
        expect![[r#"Literal@0..5
  FalseKw@0..5 "false""#]]
        .assert_eq(&parse("false", |p| {
            atom_expr(p);
        }));
    }

    #[test]
    fn parse_string() {
        expect![[r#"Literal@0..5
  String@0..5 "'abc'""#]]
        .assert_eq(&parse("'abc'", |p| {
            atom_expr(p);
        }));
    }

    #[test]
    fn parse_integer() {
        expect![[r#"Literal@0..2
  Integer@0..2 "42""#]]
        .assert_eq(&parse("42", |p| {
            atom_expr(p);
        }));
    }
}
