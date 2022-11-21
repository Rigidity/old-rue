use lexer::Lexer;
use parser::{Output, Parser};
use syntax::SyntaxNode;

fn main() {
    let source = "hello";
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let Output { green_node, .. } = Parser::parse(&tokens);
    let syntax_node = SyntaxNode::new_root(green_node);
    let debug_tree = format!("{syntax_node:#?}");
    println!("{}", &debug_tree[0..debug_tree.len() - 1]);
}
