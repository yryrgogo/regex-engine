use std::collections::HashSet;
use crate::automaton::State;

#[derive(Debug, Clone)]
pub struct DFA {
    pub start: State,
    pub accepts: HashSet<State>,
}

impl DFA {
    pub fn transition(&self, prev_state: State, input: String) {
        if input.len() == 0 {
            panic!("transition with empty input is not allowed");
        }
        todo!("transition {:?} with {}", prev_state, input);
    }
}
