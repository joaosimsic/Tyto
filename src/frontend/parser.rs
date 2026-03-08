use super::ast::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"]
pub struct TytoParser;

pub fn parse_dsl(input: &str) -> Result<TytoProgram, pest::error::Error<Rule>> {
    let mut pairs = TytoParser::parse(Rule::file, input)?;
    let mut states = Vec::new();
    let mut context = None;

    if let Some(file_pair) = pairs.next() {
        for pair in file_pair.into_inner() {
            match pair.as_rule() {
                Rule::context_block => {
                    let mut fields = Vec::new();
                    for field_item in pair.into_inner() {
                        let mut field_inner = field_item.into_inner();
                        let field_name = field_inner.next().unwrap().as_str().to_string();
                        let field_type_str = field_inner.next().unwrap().as_str();
                        fields.push(Field {
                            name: field_name,
                            field_type: TytoType::from_str(field_type_str),
                        });
                    }
                    context = Some(DataBlock { fields });
                }
                Rule::state => {
                    states.push(parse_state(pair));
                }
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }
    }

    Ok(TytoProgram { context, states })
}

fn parse_state(pair: pest::iterators::Pair<Rule>) -> State {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut transitions = Vec::new();
    let mut data = None;
    let mut is_terminal = false;

    for item in inner {
        match item.as_rule() {
            Rule::transition => {
                let trans_inner = item.into_inner();
                let mut transition_type = TransitionType::Default;
                let mut event = String::new();
                let mut target = String::new();

                for trans_part in trans_inner {
                    match trans_part.as_rule() {
                        Rule::transition_type => {
                            transition_type = match trans_part.as_str() {
                                "success" => TransitionType::Success,
                                "recoverable" => TransitionType::Recoverable,
                                "fatal" => TransitionType::Fatal,
                                _ => TransitionType::Default,
                            };
                        }
                        Rule::ident => {
                            if event.is_empty() {
                                event = trans_part.as_str().to_string();
                            } else {
                                target = trans_part.as_str().to_string();
                            }
                        }
                        _ => {}
                    }
                }

                transitions.push(Transition {
                    transition_type,
                    event,
                    target,
                });
            }
            Rule::data_block => {
                let mut fields = Vec::new();
                for field_item in item.into_inner() {
                    if field_item.as_rule() == Rule::field {
                        let mut field_inner = field_item.into_inner();
                        let field_name = field_inner.next().unwrap().as_str().to_string();
                        let field_type_str = field_inner.next().unwrap().as_str();
                        fields.push(Field {
                            name: field_name,
                            field_type: TytoType::from_str(field_type_str),
                        });
                    }
                }
                data = Some(DataBlock { fields });
            }
            Rule::terminal_flag => {
                is_terminal = true;
            }
            _ => {}
        }
    }
    State {
        name,
        transitions,
        data,
        is_terminal,
    }
}
