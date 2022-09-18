use super::automaton::{FiniteAutomaton, State};

struct AutomatonRuntime {
    automaton: FiniteAutomaton,
    current_state: State,
}

impl AutomatonRuntime {
    fn new(automaton: FiniteAutomaton) -> Self {
        match automaton {
            FiniteAutomaton::DFA(dfa) => Self {
                automaton: FiniteAutomaton::DFA(dfa.clone()),
                current_state: dfa.start,
            },
            FiniteAutomaton::NFA(nfa) => Self {
                automaton: FiniteAutomaton::NFA(nfa.clone()),
                current_state: nfa.start,
            },
        }
    }

    fn run(&self, input: String) -> bool {
        for c in input.chars() {
            match &self.automaton {
                FiniteAutomaton::DFA(dfa) => dfa.transition(self.current_state, c.to_string()),
                FiniteAutomaton::NFA(nfa) => nfa.transition(self.current_state, c.to_string()),
            }
        }
        self.is_accept()
    }

    fn do_transition(&self, input: String) {
        match &self.automaton {
            FiniteAutomaton::DFA(dfa) => {
                dfa.transition(self.current_state.clone(), input);
            }
            FiniteAutomaton::NFA(nfa) => {
                nfa.transition(self.current_state.clone(), input);
            }
        }
    }

    fn is_accept(&self) -> bool {
        match &self.automaton {
            FiniteAutomaton::DFA(dfa) => dfa.accepts.contains(&self.current_state),
            FiniteAutomaton::NFA(nfa) => nfa.accepts.contains(&self.current_state),
        }
    }
}
