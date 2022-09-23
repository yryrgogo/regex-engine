use super::{dfa::DFA, State};

#[derive(Debug)]
struct Runtime {
    dfa: DFA,
    current_state: State,
}

impl Runtime {
    fn new(dfa: DFA) -> Self {
        Self {
            dfa: dfa.clone(),
            current_state: dfa.start,
        }
    }

    fn run(&self, input: String) -> bool {
        for c in input.chars() {
            self.dfa.transition(self.current_state, c.to_string());
        }
        self.is_accept()
    }

    fn do_transition(&self, input: String) {
        self.dfa.transition(self.current_state.clone(), input);
    }

    fn is_accept(&self) -> bool {
        self.dfa.accepts.contains(&self.current_state)
    }
}
