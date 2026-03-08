use std::collections::HashSet;

use crate::frontend::ast::TransitionType;

use super::graph::StateGraph;
use petgraph::Direction::{self};

pub struct Validator;

impl Validator {
    pub fn validate(state_graph: &StateGraph) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let graph = &state_graph.graph;

        let roots: Vec<_> = graph
            .node_indices()
            .filter(|&node_idx| graph.edges_directed(node_idx, Direction::Incoming).count() == 0)
            .collect();

        if roots.is_empty() {
            errors.push(
                "Invalid Arquitecture: No initial state found (cyclic graph with no entry point)."
                    .to_string(),
            );
        } else if roots.len() > 1 {
            let root_names: Vec<_> = roots.iter().map(|&idx| graph[idx].as_str()).collect();
            errors.push(format!(
                "Invalid Architecture: Multiple initial states (roots) detected: {:?}",
                root_names
            ));
        }

        for node_idx in graph.node_indices() {
            let state_name = &graph[node_idx];
            let is_terminal = state_graph.terminal_nodes.contains(&node_idx);

            let outgoing_edges: Vec<_> = graph
                .edges_directed(node_idx, Direction::Outgoing)
                .collect();
            let incoming_count = graph.edges_directed(node_idx, Direction::Incoming).count();

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

            if incoming_count == 0 && roots.first() != Some(&node_idx) {
                errors.push(format!(
                    "Orphan State: State '{}' is unreachable. No transitions point to it.",
                    state_name
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
