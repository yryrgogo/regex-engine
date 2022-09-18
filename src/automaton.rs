use crate::{dfa::DFA, nfa::NFA};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum FiniteAutomaton {
    NFA(NFA),
    DFA(DFA),
}
