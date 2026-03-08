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
    pub field_type: String,
}
