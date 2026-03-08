use crate::frontend::ast::TytoProgram;

pub mod rust;
pub mod typescript;
pub mod mermaid;

pub trait Generator {
    fn generate(&self, program: &TytoProgram) -> String;
    fn extension(&self) -> &'static str;
}

pub fn get_generator(lang: &str) -> Option<Box<dyn Generator>> {
    match lang {
        "typescript" => Some(Box::new(typescript::TypeScriptGenerator)),
        "rust" => Some(Box::new(rust::RustGenerator)),
        "mermaid" => Some(Box::new(mermaid::MermaidGenerator)),
        _ => None,
    }
}
