use lexer::Lexer;

fn main() {
    let source = "3 + 2";
    let tokens = Lexer::new(source).collect::<Vec<_>>();
    dbg!(tokens);
}
