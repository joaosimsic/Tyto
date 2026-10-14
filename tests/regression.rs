use tyto::middle::{StateGraph, Validator};

fn first_error(source: &str) -> String {
    let program = tyto::frontend::parse_dsl(source).expect("fixture must parse");
    let graph = StateGraph::from_ast(&program).expect("graph must build");
    Validator::validate(&graph)
        .expect_err("fixture must fail validation")
        .into_iter()
        .next()
        .unwrap_or_default()
}

#[test]
fn empty_machine_is_rejected() {
    assert!(first_error(include_str!("fixtures/bad_empty.ty")).contains("Invalid Architecture"));
}

#[test]
fn deadlock_is_detected() {
    assert!(first_error(include_str!("fixtures/bad_deadlock.ty")).contains("Deadlock Detected"));
}

#[test]
fn terminal_violation_is_detected() {
    assert!(first_error(include_str!("fixtures/bad_terminal.ty")).contains("Terminal Violation"));
}

#[test]
fn orphan_state_is_detected() {
    assert!(first_error(include_str!("fixtures/bad_orphan.ty")).contains("Orphan State"));
}

#[test]
fn non_determinism_is_detected() {
    assert!(first_error(include_str!("fixtures/bad_nondeterminism.ty")).contains("Non-determinism"));
}

#[test]
fn resilience_failure_is_detected() {
    assert!(first_error(include_str!("fixtures/bad_resilience.ty")).contains("Resilience Failure"));
}

#[test]
fn nonexistent_target_is_reported() {
    let program = tyto::frontend::parse_dsl(include_str!("fixtures/bad_nonexistent.ty")).unwrap();
    let error = StateGraph::from_ast(&program).expect_err("must reject missing target");
    assert!(error.contains("non-existent state"));
}

#[test]
fn valid_machines_pass_validation() {
    for source in [
        include_str!("fixtures/order.ty"),
        include_str!("fixtures/complex.ty"),
    ] {
        let program = tyto::frontend::parse_dsl(source).unwrap();
        let graph = StateGraph::from_ast(&program).unwrap();
        assert!(Validator::validate(&graph).is_ok());
    }
}
