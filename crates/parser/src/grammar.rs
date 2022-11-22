use syntax::SyntaxKind;

use crate::{parser::CompletedMarker, Parser};

use self::expr::expr;

mod expr;

pub(super) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    expr(p);
    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
fn parse<F>(source: &str, f: F) -> String
where
    F: FnOnce(&mut Parser),
{
    use crate::Input;
    use lexer::Lexer;

    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let mut parser = Parser::new(Input::from_tokens(&tokens));

    f(&mut parser);
    parser.parse(&tokens).debug_tree()
}

#[cfg(test)]
mod tests {}
