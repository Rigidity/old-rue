use syntax::SyntaxKind;

use crate::{marker::CompletedMarker, Parser};

use self::expr::expr;

mod expr;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    expr(p);
    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
fn parse(source: &str) -> String {
    use lexer::Lexer;

    let tokens = Lexer::new(source).collect::<Vec<_>>();
    Parser::parse(&tokens).debug_tree()
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn parse_root() {
        expect![[r#"Root@0..0"#]].assert_eq(&parse(""));
    }
}
