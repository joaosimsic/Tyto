use std::collections::HashSet;

use crate::frontend::ast::TransitionType;

use super::graph::StateGraph;
use petgraph::{
    graph::NodeIndex,
    visit::Dfs,
    Direction::{self},
};

pub struct Validator;

impl Validator {
    pub fn validate(state_graph: &StateGraph) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let graph = &state_graph.graph;

        if graph.node_count() == 0 {
            errors.push("Invalid Architecture: The state machine is empty.".to_string());
            return Err(errors);
        }

        let root_idx = NodeIndex::new(0);

        let mut reachable = HashSet::new();
        let mut dfs = Dfs::new(graph, root_idx);
        while let Some(nx) = dfs.next(graph) {
            reachable.insert(nx);
        }

        for node_idx in graph.node_indices() {
            let state_name = &graph[node_idx];
            let is_terminal = state_graph.terminal_nodes.contains(&node_idx);

            let outgoing_edges: Vec<_> = graph
                .edges_directed(node_idx, Direction::Outgoing)
                .collect();

            if is_terminal && !outgoing_edges.is_empty() {
                errors.push(format!(
                "Terminal Violation: State '{}' is marked as terminal but has outgoing transitions.",
                state_name
            ));
            }

            if outgoing_edges.is_empty() && !is_terminal {
                errors.push(format!(
                "Deadlock Detected: State '{}' has no outgoing transitions and is not explicitly marked as 'terminal'.",
                state_name
            ));
            }

            if !reachable.contains(&node_idx) {
                errors.push(format!(
                    "Orphan State: State '{}' is unreachable from the initial state '{}'.",
                    state_name, graph[root_idx]
                ));
            }

            let mut seen_events = HashSet::new();
            for edge in &outgoing_edges {
                let transition = edge.weight();
                if !seen_events.insert(&transition.event) {
                    errors.push(format!(
                    "Non-determinism: State '{}' has multiple transitions responding to the same event '{}'.",
                        state_name, transition.event
                ));
                }
            }

            let has_success = outgoing_edges
                .iter()
                .any(|e| e.weight().transition_type == TransitionType::Success);

            let has_failure_handling = outgoing_edges.iter().any(|e| {
                matches!(
                    e.weight().transition_type,
                    TransitionType::Recoverable | TransitionType::Fatal
                )
            });

            if has_success && !has_failure_handling {
                errors.push(format!(
                "Resilience Failure: State '{}' has a 'Success' route indicating a critical operation, but no error routes ('Recoverable' or 'Fatal') are mapped.",
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
