use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = include_str!("hello_world.rue");
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let output = Parser::parse_tokens(&tokens);
    println!("{}", output.debug_tree());
}
