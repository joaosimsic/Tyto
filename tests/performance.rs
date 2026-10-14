use std::time::{Duration, Instant};

use tyto::middle::{StateGraph, Validator};

fn linear_machine(states: usize) -> String {
    let mut source = String::new();
    for i in 0..states {
        if i + 1 < states {
            source.push_str(&format!("state S{} {{ on Go -> S{}; }}\n", i, i + 1));
        } else {
            source.push_str(&format!("state S{} {{ terminal; }}\n", i));
        }
    }
    source
}

fn validate_source(source: &str) {
    let program = tyto::frontend::parse_dsl(source).expect("synthetic machine must parse");
    let graph = StateGraph::from_ast(&program).expect("synthetic graph must build");
    Validator::validate(&graph).expect("synthetic machine must be valid");
}

#[test]
fn validation_scales_linearly_and_stays_under_budget() {
    for states in [10, 50, 100] {
        let source = linear_machine(states);

        let start = Instant::now();
        validate_source(&source);
        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_millis(100),
            "validation of {} states took {:?} (> 100ms)",
            states,
            elapsed
        );
    }
}

#[test]
fn parsing_scales_for_large_machines() {
    let source = linear_machine(100);
    let program = tyto::frontend::parse_dsl(&source).expect("must parse");
    assert_eq!(program.states.len(), 100);
}
