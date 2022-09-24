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
}

impl NFA {
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

    pub fn render_nfa_graph(&self, filename: &str) {
        let mut output = "".to_string();
        fn add_prologue(output: &mut String) {
            output.push_str("digraph G {\n");
            output.push_str("  rankdir = LR;\n");
            output.push_str("  node [shape = circle];\n");
        }

        fn add_epilogue(output: &mut String) {
            output.push_str("}");
        }

        fn add_node(output: &mut String, node_id: usize, is_accept: bool) {
            output.push_str(&format!(
                "  {} [label = \"{}\";shape = \"{}\"];\n",
                node_id,
                node_id,
                if is_accept { "doublecircle" } else { "circle" }
            ));
        }

        fn add_edge(output: &mut String, from: usize, to: usize, label: &str) {
            output.push_str(&format!("  {} -> {} [label = \"{}\"];\n", from, to, label));
        }

        add_prologue(&mut output);

        let mut edge_output = "".to_string();
        let mut node_set = BTreeSet::<State>::new();

        for (k, v) in self
            .map
            .as_ref()
            .unwrap_or_else(|| panic!("map is not found"))
        {
            for state in v {
                add_edge(&mut edge_output, k.current_state.id, state.id, &k.input);
                node_set.insert(k.current_state);
                node_set.insert(state.clone());
            }
            k.current_state;
        }

        for state in &node_set {
            let is_accept = self
                .accepts
                .as_ref()
                .unwrap_or_else(|| panic!("accepts is not found"))
                .contains(state);
            add_node(&mut output, state.id, is_accept);
        }
        output.push_str(&edge_output);

        add_epilogue(&mut output);

        use std::fs::File;
        use std::io::{BufWriter, Write};

        let f = File::create(&filename).unwrap();
        let mut writer = BufWriter::new(f);
        writer.write_all(output.as_bytes()).unwrap();
    }
}
