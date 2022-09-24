use std::collections::BTreeSet;

pub mod dfa;
pub mod nfa;
pub mod runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct State {
    pub id: usize,
}
impl State {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub type StateSet = BTreeSet<State>;
