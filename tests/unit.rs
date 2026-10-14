use tyto::frontend::ast::{BaseType, TytoType};
use tyto::frontend::parse_dsl;

const ORDER: &str = include_str!("fixtures/order.ty");
const COMPLEX: &str = include_str!("fixtures/complex.ty");

#[test]
fn parses_context_and_states() {
    let program = parse_dsl(ORDER).expect("order.ty must parse");

    let context = program
        .context
        .as_ref()
        .expect("context block must be present");
    assert_eq!(context.fields.len(), 2);
    assert_eq!(context.fields[0].name, "order_id");

    assert_eq!(program.states.len(), 3);

    let pending = program.find_state("Pending").expect("Pending state");
    assert!(!pending.is_terminal);
    assert_eq!(pending.transitions.len(), 2);

    let paid = program.find_state("Paid").expect("Paid state");
    assert!(paid.is_terminal);
}

#[test]
fn parses_optional_and_array_types() {
    assert_eq!(
        TytoType::parse("String?").unwrap(),
        TytoType::Optional(Box::new(TytoType::Base(BaseType::String)))
    );
    assert_eq!(
        TytoType::parse("String[]").unwrap(),
        TytoType::Array(Box::new(TytoType::Base(BaseType::String)))
    );
    assert!(TytoType::parse("Unknown").is_err());
}

#[test]
fn context_is_inherited_by_states() {
    let program = parse_dsl(ORDER).unwrap();
    let paid = program.find_state("Paid").unwrap();

    let names: Vec<String> = paid
        .all_fields(&program)
        .iter()
        .map(|f| f.name.clone())
        .collect();

    assert!(names.contains(&"order_id".to_string()));
    assert!(names.contains(&"transaction_id".to_string()));
}

#[test]
fn parses_complex_machine_with_recovery() {
    let program = parse_dsl(COMPLEX).expect("complex.ty must parse");
    let pending = program.find_state("Pending").unwrap();
    assert_eq!(pending.transitions.len(), 3);

    let shipped = program.find_state("Shipped").unwrap();
    assert!(shipped.is_terminal);
    let shipped_fields = shipped.all_fields(&program);
    assert!(shipped_fields.iter().any(|f| f.name == "items"));
    assert!(shipped_fields.iter().any(|f| f.name == "tracking_number"));
}
