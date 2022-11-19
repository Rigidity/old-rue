use lexer::Lexer;

fn main() {
    let source = "hello";
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    dbg!(tokens);
}
