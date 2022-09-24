use crate::automaton::runtime::Runtime;
use automaton::nfa::NFA;
use compiler::{lexer::Lexer, parser::Parser};
use viz::graph_viz::GraphViz;

mod automaton;
mod compiler;
mod viz;

pub struct RegExp {
    nfa: NFA,
}

impl RegExp {
    pub fn new(regex: String) -> Self {
        let mut lexer = Lexer::new(regex.clone());
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let nfa = parser.parse();

        // TODO: ここで DFA に変換したいが、関数内で作成した NFA を DFA の transition (closure) で使用しており、
        // その関数が NFA の lifetime を持っているため、NFA が drop されてしまう

        Self { nfa }
    }

    pub fn matches(&self, input: String) -> bool {
        let dfa = self.nfa.nfa2dfa();
        let mut runtime = Runtime::new(&dfa);
        runtime.run(input)
    }

    pub fn render_nfa(&self, filename: &str) {
        let viz = GraphViz {};
        viz.render_nfa_graph(&self.nfa, filename);
    }
}
