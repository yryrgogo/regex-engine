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
    fn transition(&self, map: HashMap<NFAInput, StateSet>, input: NFAInput) -> HashSet<State> {
        if let Some(states) = map.get(&input) {
            states.clone()
        } else {
            panic!("no state found for {:?}", input);
        }
    }
}
