use std::collections::BTreeSet;

use crate::automaton::{nfa::NFA, State};

pub struct GraphViz {}

impl GraphViz {
    fn add_prologue(&self, output: &mut String) {
        output.push_str("digraph G {\n");
        output.push_str("  rankdir = LR;\n");
        output.push_str("  node [shape = circle];\n");
    }

    fn add_epilogue(&self, output: &mut String) {
        output.push_str("}");
    }

    fn add_node(&self, output: &mut String, node_id: usize, is_accept: bool) {
        output.push_str(&format!(
            "  {} [label = \"{}\";shape = \"{}\"];\n",
            node_id,
            node_id,
            if is_accept { "doublecircle" } else { "circle" }
        ));
    }

    fn add_edge(&self, output: &mut String, from: usize, to: usize, label: &str) {
        output.push_str(&format!("  {} -> {} [label = \"{}\"];\n", from, to, label));
    }

    pub fn render_nfa_graph(&self, nfa: &NFA, filename: &str) {
        let mut output = "".to_string();

        self.add_prologue(&mut output);

        let mut edge_output = "".to_string();
        let mut node_set = BTreeSet::<State>::new();

        for (k, v) in nfa
            .map
            .as_ref()
            .unwrap_or_else(|| panic!("map is not found"))
        {
            for state in v {
                self.add_edge(&mut edge_output, k.current_state.id, state.id, &k.input);
                node_set.insert(k.current_state);
                node_set.insert(state.clone());
            }
            k.current_state;
        }

        for state in &node_set {
            let is_accept = nfa
                .accepts
                .as_ref()
                .unwrap_or_else(|| panic!("accepts is not found"))
                .contains(state);
            self.add_node(&mut output, state.id, is_accept);
        }
        output.push_str(&edge_output);

        self.add_epilogue(&mut output);

        use std::fs::File;
        use std::io::{BufWriter, Write};

        let f = File::create(&filename).unwrap();
        let mut writer = BufWriter::new(f);
        writer.write_all(output.as_bytes()).unwrap();
    }
}
