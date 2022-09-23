use std::collections::{HashMap, HashSet};

use crate::compiler::fragment::NFAInput;

use super::{State, StateSet, Transition};

#[derive(Debug, Clone)]
pub struct NFA {
    pub start: Option<State>,
    pub accepts: Option<StateSet>,
    pub map: Option<HashMap<NFAInput, StateSet>>,
}

impl Transition for NFA {
    fn transition(&self, input: &NFAInput) -> StateSet {
        if let Some(states) = self.map.as_ref().unwrap().get(input) {
            states.clone()
        } else {
            panic!("no state found for {:?}", input);
        }
    }
}

impl NFA {
    pub fn epsilon_expand(&self, states: StateSet) -> StateSet {
        let mut que = HashSet::<State>::new();
        que.extend(states);
        let mut done = HashSet::<State>::new();

        while !que.is_empty() {
            let state = que.iter().next().unwrap().clone();
            que.remove(&state);
            done.insert(state);

            let input = NFAInput::new("".to_string(), state);
            for next_state in self.transition(&input) {
                if !done.contains(&next_state) {
                    que.insert(next_state);
                }
            }
        }

        done
    }
}
