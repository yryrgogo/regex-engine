use std::collections::HashSet;

#[derive(Debug, Clone)]
struct State {}

#[derive(Debug, Clone)]
struct NFA {
    start: State,
    accepts: HashSet<State>,
}

impl NFA {
    fn transition(prev_state: State, input: String) {
        todo!("transition {:?} with {}", prev_state, input);
    }
}
