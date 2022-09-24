use compiler::{lexer::Lexer, parser::Parser};

use crate::{
    automaton::runtime::Runtime,
    viz::graph_viz::{render_to, Edges},
};

use std::fs::File;

mod automaton;
mod compiler;
mod viz;

fn main() {
    let mut lexer = Lexer::new("a|(bc)*".to_string());
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let nfa = parser.parse();

    println!("{:#?}", nfa);

    let mut f = File::create("example1.dot").unwrap();
    let edges = Edges(vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4), (4, 4)]);
    render_to(edges, &mut f);

    let dfa = nfa.nfa2dfa();
    let mut runtime = Runtime::new(&dfa);

    let input = "abcbc".to_string();
    runtime.run(input);
}
