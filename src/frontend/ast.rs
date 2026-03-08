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
