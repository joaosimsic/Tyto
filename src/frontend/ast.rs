#[derive(Debug, Clone)]
pub struct TytoProgram {
    pub context: Option<DataBlock>,
    pub states: Vec<State>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub transitions: Vec<Transition>,
    pub data: Option<DataBlock>,
    pub is_terminal: bool,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub transition_type: TransitionType,
    pub event: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct DataBlock {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: TytoType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    Success,
    Recoverable,
    Fatal,
    Default,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    String,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TytoType {
    Base(BaseType),
    Array(Box<TytoType>),
    Optional(Box<TytoType>),
}

impl TytoType {
    pub fn from_str(s: &str) -> Self {
        if s.ends_with('?') {
            let inner_type = Self::from_str(&s[..s.len() - 1]);
            return TytoType::Optional(Box::new(inner_type));
        }

        if s.ends_with("[]") {
            let inner_type = Self::from_str(&s[..s.len() - 2]);
            return TytoType::Array(Box::new(inner_type));
        }

        match s {
            "String" => TytoType::Base(BaseType::String),
            "Int" => TytoType::Base(BaseType::Int),
            "Float" => TytoType::Base(BaseType::Float),
            "Bool" => TytoType::Base(BaseType::Bool),
            _ => panic!("Unknown type: {}", s),
        }
    }
}
