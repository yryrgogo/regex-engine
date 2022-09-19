use compiler::{lexer::Lexer, parser::Parser};

mod automaton;
mod compiler;

fn main() {
    let mut lexer = Lexer::new("a|(bc)*".to_string());
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let node = parser.parse();

    println!("{:#?}", node);
}
