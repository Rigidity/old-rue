use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let tokens = Lexer::new(&input).collect::<Vec<_>>();
        let output = Parser::parse(&tokens);
        println!("{}", output.debug_tree());

        input.clear();
    }
}
