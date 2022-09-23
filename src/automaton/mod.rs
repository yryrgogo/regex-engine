use std::collections::{HashMap, HashSet};

use crate::compiler::fragment::NFAInput;

pub mod dfa;
pub mod nfa;
pub mod runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {}
impl State {
    pub fn new() -> Self {
        Self {}
    }
}

pub type StateSet = HashSet<State>;

pub trait Transition {
    fn transition(&self, map: HashMap<NFAInput, StateSet>, input: NFAInput) -> HashSet<State>;
}
