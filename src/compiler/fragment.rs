use std::collections::{BTreeSet, HashMap};

use crate::automaton::{nfa::NFA, State, StateSet};

#[derive(Debug, PartialEq, Eq)]
pub struct NFAFragment {
    pub start: Option<State>,
    pub accepts: Option<StateSet>,
    pub map: HashMap<NFAInput, StateSet>,
}

impl Default for NFAFragment {
    fn default() -> Self {
        Self {
            start: None,
            accepts: None,
            map: HashMap::new(),
        }
    }
}

impl NFAFragment {
    pub fn new(start: State, accepts: StateSet, map: Option<HashMap<NFAInput, StateSet>>) -> Self {
        if let Some(m) = map {
            Self {
                start: Some(start),
                accepts: Some(accepts),
                map: m,
            }
        } else {
            Self {
                start: Some(start),
                accepts: Some(accepts),
                map: HashMap::new(),
            }
        }
    }

    pub fn connect(&mut self, nfa_input: NFAInput, next: State) {
        self.map
            .entry(nfa_input)
            .or_insert_with(|| {
                let mut set = BTreeSet::new();
                set.insert(next);
                set
            })
            .insert(next);
    }

    pub fn union(&mut self, fragment: &Self) {
        for (input, states) in fragment.map.iter() {
            self.map.insert(input.clone(), states.clone());
        }
    }

    pub fn new_skeleton(&self) -> Self {
        let new_fragment = NFAFragment {
            start: None,
            accepts: None,
            map: self.map.clone(),
        };
        new_fragment
    }

    pub fn build(&self) -> NFA {
        let map = self.map.clone();
        NFA {
            start: self.start,
            accepts: self.accepts.clone(),
            map: Some(map),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct NFAInput {
    pub input: String,
    pub current_state: State,
}

impl NFAInput {
    pub fn new(input: String, current_state: State) -> Self {
        Self {
            input,
            current_state,
        }
    }
}
