use std::collections::HashMap;

use petgraph::graph::{Graph, NodeIndex};

use crate::frontend::ast::{Transition, TytoProgram};

#[derive(Debug)]
pub struct StateGraph {
    pub graph: Graph<String, Transition>,
    pub indices: HashMap<String, NodeIndex>,
    pub terminal_nodes: Vec<NodeIndex>,
}

impl StateGraph {
    pub fn from_ast(program: &TytoProgram) -> Result<Self, String> {
        let mut graph = Graph::<String, Transition>::new();
        let mut indices = HashMap::new();
        let mut terminal_nodes = Vec::new();

        for state in &program.states {
            let node_idx = graph.add_node(state.name.clone());
            indices.insert(state.name.clone(), node_idx);

            if state.is_terminal {
                terminal_nodes.push(node_idx);
            }
        }

        for state in &program.states {
            let source_idx = indices.get(&state.name).unwrap();

            for transition in &state.transitions {
                let target_idx = indices.get(&transition.target).ok_or_else(|| {
                    format!(
                        "Semantic Error: The state '{}' attempts to transition to a non-existent state '{}'.",
                            state.name, transition.target
                    )
                })?;

                graph.add_edge(*source_idx, *target_idx, transition.clone());
            }
        }

        Ok(StateGraph {
            graph,
            indices,
            terminal_nodes,
        })
    }
}
