use compiler::{lexer::Lexer, parser::Parser};

use crate::automaton::runtime::Runtime;

mod automaton;
mod compiler;

fn main() {
    let mut lexer = Lexer::new("a|(bc)*".to_string());
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let nfa = parser.parse();

    nfa.render_nfa_graph("nfa.dot");
    println!("{:#?}", nfa);

    let dfa = nfa.nfa2dfa();
    let mut runtime = Runtime::new(&dfa);

    let input = "abcbc".to_string();
    runtime.run(input);
}
