use std::collections::HashSet;

use super::automaton::State;

#[derive(Debug, Clone)]
pub struct NFA {
    pub start: State,
    pub accepts: HashSet<State>,
}

impl NFA {
    pub fn transition(&self, prev_state: State, input: String) {
        todo!("transition {:?} with {}", prev_state, input);
    }
}
