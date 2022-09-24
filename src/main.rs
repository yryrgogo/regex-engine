use compiler::{lexer::Lexer, parser::Parser};

use crate::automaton::runtime::Runtime;

mod automaton;
mod compiler;

fn main() {
    let mut lexer = Lexer::new("a|(bc)*".to_string());
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let nfa = parser.parse();

    nfa.render_nfa_graph("nfa.dot");

    let dfa = nfa.nfa2dfa();
    let mut runtime = Runtime::new(&dfa);

    let input = "ab".to_string();
    let result = runtime.run(input);
    println!("result: {}", result);
}
