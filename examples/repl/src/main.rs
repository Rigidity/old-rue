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

        let tokens = Lexer::new(&input.trim()).collect::<Vec<_>>();
        let output = Parser::parse_tokens(&tokens);

        println!("{}", output.debug_tree());

        for error in output.errors {
            println!("{}", error);
        }

        input.clear();
    }
}
