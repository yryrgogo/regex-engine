use crate::automaton::State;

pub mod ast;
pub mod fragment;
pub mod lexer;
pub mod parser;
pub mod token;

pub struct Context {
    pub current_state: usize,
}

impl Default for Context {
    fn default() -> Self {
        Self { current_state: 0 }
    }
}

impl Context {
    pub fn new_state(&mut self) -> State {
        self.current_state += 1;
        State::new(self.current_state - 1)
    }
}
