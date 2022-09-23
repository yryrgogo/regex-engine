use crate::automaton::State;

pub mod ast;
pub mod fragment;
pub mod lexer;
pub mod parser;
pub mod token;

pub struct Context {
    pub current_state: usize,
}

impl Context {
    pub fn new_state(&mut self) -> State {
        self.current_state += 1;
        State::new()
    }
}
