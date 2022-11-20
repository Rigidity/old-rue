use syntax::SyntaxKind;

use crate::{marker::CompletedMarker, Parser};

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
fn parse(source: &str) -> String {
    use lexer::Lexer;
    use syntax::SyntaxNode;

    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let output = Parser::new(&tokens).parse();
    let syntax_node = SyntaxNode::new_root(output.green_node);
    let debug_tree = format!("{:#?}", syntax_node);

    debug_tree[0..debug_tree.len() - 1].to_string()
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
