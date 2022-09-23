use std::collections::{HashMap, HashSet};

use crate::compiler::fragment::NFAInput;

pub mod dfa;
pub mod nfa;
pub mod runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pub id: usize,
}
impl State {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub type StateSet = HashSet<State>;

pub trait Transition {
    fn transition(&self, input: &NFAInput) -> HashSet<State>;
}
