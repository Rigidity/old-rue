use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = "3 + 2";
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let green_node = parser.parse();
    println!("{green_node:?}");
}
