use std::collections::{HashMap, HashSet};

use crate::automaton::{nfa::NFA, State, StateSet};

#[derive(Debug)]
pub struct NFAFragment {
    pub start: Option<State>,
    pub accepts: Option<StateSet>,
    pub map: HashMap<NFAInput, HashSet<State>>,
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
    pub fn new(start: State, accepts: StateSet) -> Self {
        Self {
            start: Some(start),
            accepts: Some(accepts),
            map: HashMap::new(),
        }
    }

    pub fn connect(&mut self, nfa_input: NFAInput, next: State) {
        let mut set = HashSet::new();
        set.insert(next);
        self.map.insert(nfa_input, set);
    }

    pub fn union(&self, fragment: Self) -> Self {
        let mut new_fragment = self.new_skeleton();
        for (input, states) in fragment.map {
            new_fragment.map.insert(input, states);
        }
        new_fragment
    }

    fn new_skeleton(&self) -> Self {
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
