use super::ast::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"]
pub struct TytoParser;

#[derive(Debug)]
pub enum ParseError {
    Pest(pest::error::Error<Rule>),
    Type(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Pest(e) => write!(f, "{}", e),
            ParseError::Type(e) => write!(f, "{}", e),
        }
    }
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        ParseError::Pest(e)
    }
}

pub fn parse_dsl(input: &str) -> Result<TytoProgram, ParseError> {
    let mut pairs = TytoParser::parse(Rule::file, input)?;
    let mut states = Vec::new();
    let mut context = None;

    if let Some(file_pair) = pairs.next() {
        for pair in file_pair.into_inner() {
            match pair.as_rule() {
                Rule::context_block => {
                    context = Some(parse_data_block(pair)?);
                }
                Rule::state => {
                    states.push(parse_state(pair)?);
                }
                Rule::EOI => break,
                _ => {}
            }
        }
    }

    Ok(TytoProgram { context, states })
}

fn parse_field(pair: pest::iterators::Pair<Rule>) -> Result<Field, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let type_str = inner.next().unwrap().as_str();
    let field_type = TytoType::parse(type_str).map_err(ParseError::Type)?;
    Ok(Field { name, field_type })
}

fn parse_data_block(pair: pest::iterators::Pair<Rule>) -> Result<DataBlock, ParseError> {
    let mut fields = Vec::new();
    for field_item in pair.into_inner() {
        if field_item.as_rule() == Rule::field {
            fields.push(parse_field(field_item)?);
        }
    }
    Ok(DataBlock { fields })
}

fn parse_state(pair: pest::iterators::Pair<Rule>) -> Result<State, ParseError> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut transitions = Vec::new();
    let mut data = None;
    let mut is_terminal = false;

    for item in inner {
        match item.as_rule() {
            Rule::transition => {
                transitions.push(parse_transition(item));
            }
            Rule::data_block => {
                data = Some(parse_data_block(item)?);
            }
            Rule::terminal_flag => {
                is_terminal = true;
            }
            _ => {}
        }
    }

    Ok(State {
        name,
        transitions,
        data,
        is_terminal,
    })
}

fn parse_transition(pair: pest::iterators::Pair<Rule>) -> Transition {
    let mut transition_type = TransitionType::Default;
    let mut event = String::new();
    let mut target = String::new();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::transition_type => {
                transition_type = match part.as_str() {
                    "success" => TransitionType::Success,
                    "recoverable" => TransitionType::Recoverable,
                    "fatal" => TransitionType::Fatal,
                    _ => TransitionType::Default,
                };
            }
            Rule::ident => {
                if event.is_empty() {
                    event = part.as_str().to_string();
                } else {
                    target = part.as_str().to_string();
                }
            }
            _ => {}
        }
    }

    Transition {
        transition_type,
        event,
        target,
    }
}
