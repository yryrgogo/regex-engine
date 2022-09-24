use std::collections::{BTreeSet, HashMap};

use crate::compiler::fragment::NFAInput;

use super::{dfa::DFA, State, StateSet};

#[derive(Debug, Clone)]
pub struct NFA {
    pub start: Option<State>,
    pub accepts: Option<StateSet>,
    pub map: Option<HashMap<NFAInput, StateSet>>,
}

impl NFA {
    fn transition(&self, input: &NFAInput) -> Option<StateSet> {
        if let Some(states) = self.map.as_ref().unwrap().get(input) {
            Some(states.clone())
        } else {
            None
        }
    }

    pub fn epsilon_expand(&self, states: StateSet) -> StateSet {
        let mut que = BTreeSet::<State>::new();
        que.extend(states);
        let mut done = BTreeSet::<State>::new();

        while !que.is_empty() {
            let state = que.iter().next().unwrap().clone();
            que.remove(&state);
            done.insert(state);

            let input = NFAInput::new("".to_string(), state);
            let next_states = self.transition(&input);
            if let Some(states) = next_states {
                for next_state in states {
                    if !done.contains(&next_state) {
                        que.insert(next_state);
                    }
                }
            }
        }
        done
    }

    pub fn nfa2dfa(&self) -> DFA {
        let transition = |prev_states: &StateSet, input: String| {
            let mut new_states = BTreeSet::<State>::new();
            for state in prev_states {
                let next_states = self.transition(&NFAInput::new(input.clone(), state.clone()));
                if let Some(states) = next_states {
                    new_states.extend(states);
                }
            }
            self.epsilon_expand(new_states)
        };

        let mut tmp = BTreeSet::new();
        tmp.insert(self.start.unwrap_or_else(|| panic!("self.start is None")));
        let dfa_start = self.epsilon_expand(tmp);

        DFA {
            start: dfa_start,
            accepts: self
                .accepts
                .clone()
                .unwrap_or_else(|| panic!("self.accepts is None")),
            transition: Box::new(transition),
        }
    }
}
