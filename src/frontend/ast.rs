#[derive(Debug, Clone)]
pub struct TytoProgram {
    pub context: Option<DataBlock>,
    pub states: Vec<State>,
}

impl TytoProgram {
    pub fn find_state(&self, name: &str) -> Option<&State> {
        self.states.iter().find(|s| s.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    pub transitions: Vec<Transition>,
    pub data: Option<DataBlock>,
    pub is_terminal: bool,
}

impl State {
    pub fn all_fields(&self, program: &TytoProgram) -> Vec<Field> {
        let mut fields = Vec::new();
        if let Some(ctx) = &program.context {
            fields.extend(ctx.fields.clone());
        }
        if let Some(data) = &self.data {
            fields.extend(data.fields.clone());
        }
        fields
    }
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
    pub fn parse(s: &str) -> Result<Self, String> {
        if s.ends_with('?') {
            let inner_type = Self::parse(&s[..s.len() - 1])?;
            return Ok(TytoType::Optional(Box::new(inner_type)));
        }

        if s.ends_with("[]") {
            let inner_type = Self::parse(&s[..s.len() - 2])?;
            return Ok(TytoType::Array(Box::new(inner_type)));
        }

        match s {
            "String" => Ok(TytoType::Base(BaseType::String)),
            "Int" => Ok(TytoType::Base(BaseType::Int)),
            "Float" => Ok(TytoType::Base(BaseType::Float)),
            "Bool" => Ok(TytoType::Base(BaseType::Bool)),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}
