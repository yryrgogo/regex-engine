use std::collections::HashSet;

#[derive(Debug, Clone)]
struct State {}

#[derive(Debug, Clone)]
struct DFA {
    start: State,
    accepts: HashSet<State>,
}

impl DFA {
    fn transition(prev_state: State, input: String) {
        if input.len() == 0 {
            panic!("transition with empty input is not allowed");
        }
        todo!("transition {:?} with {}", prev_state, input);
    }
}
