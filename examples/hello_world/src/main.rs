use lexer::Lexer;
use parser::{Output, Parser};
use syntax::SyntaxNode;

fn main() {
    let source = "3 + 2";
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let Output { green_node } = parser.parse();
    let syntax_node = SyntaxNode::new_root(green_node);
    let debug_tree = format!("{syntax_node:#?}");
    println!("{}", &debug_tree[0..debug_tree.len() - 1]);
}
