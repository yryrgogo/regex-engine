use super::{dfa::DFA, StateSet};

pub struct Runtime<'a> {
    dfa: &'a DFA<'a>,
    current_state: StateSet,
}

impl<'a> Runtime<'a> {
    pub fn new(dfa: &'a DFA) -> Self {
        Self {
            dfa: dfa,
            current_state: dfa.start.clone(),
        }
    }

    pub fn run(&mut self, input: String) -> bool {
        for c in input.chars() {
            self.do_transition(c.to_string());
        }
        self.is_accept()
    }

    fn do_transition(&mut self, input: String) {
        let next_state = (self.dfa.transition)(&self.current_state, input);
        self.current_state = next_state;
    }

    fn is_accept(&self) -> bool {
        self.dfa.accepts.intersection(&self.current_state).count() > 0
    }
}
