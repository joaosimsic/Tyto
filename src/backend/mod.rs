use crate::frontend::ast::TytoProgram;

pub trait Generator {
    fn generate(&self, program: &TytoProgram) -> String;
    fn extension(&self) -> &'static str;
}

macro_rules! register_generators {
    ($($lang:expr => $module:ident::$generator:ident),* $(,)?) => {
        $(pub mod $module;)*

        pub fn get_generator(lang: &str) -> Option<Box<dyn Generator>> {
            match lang {
                $($lang => Some(Box::new($module::$generator)),)*
                _ => None,
            }
        }
    };
}

register_generators! {
    "typescript" => typescript::TypeScriptGenerator,
    "rust"       => rust::RustGenerator,
    "mermaid"    => mermaid::MermaidGenerator,
    "java"       => java::JavaGenerator,
}
