use super::graph::StateGraph;
use petgraph::Direction;

pub struct Validator;

impl Validator {
    pub fn validate(state_graph: &StateGraph) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for node_idx in state_graph.graph.node_indices() {
            let state_name = &state_graph.graph[node_idx];

            let is_terminal = state_graph.terminal_nodes.contains(&node_idx);
            let has_outgoing_edges = state_graph
                .graph
                .edges_directed(node_idx, Direction::Outgoing)
                .count()
                > 0;
            let has_ingoing_edges = state_graph
                .graph
                .edges_directed(node_idx, Direction::Incoming)
                .count()
                > 0;

            if !has_outgoing_edges && !is_terminal {
                errors.push(format!(
                "Deadlock detected: The state '{}' has no outgoing transitions and is not marked as 'terminal'.",
                    state_name
            ));
            }

            let is_first_node = node_idx.index() == 0;
            if !has_ingoing_edges && !is_first_node {
                errors.push(format!(
                "Orphan state detected: The state '{}' is inaccessible. No transitions point to it.",
                    state_name
            ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
