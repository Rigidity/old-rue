use lexer::Lexer;

fn main() {
    let source = "hello";
    let mut lexer = Lexer::new(source);
    dbg!(lexer.next());
}
