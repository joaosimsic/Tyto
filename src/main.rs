mod frontend;
mod middle;

use frontend::parse_dsl;
use middle::{StateGraph, Validator};

fn main() {
    let input_code = r#"
        state Pendente {
            data { id: String, }
        }

        state Pago {
            on Enviar -> Enviado;
        }

        state Esquecido {
            terminal;
        }

        state Enviado {
            terminal; 
        }
    "#;

    println!("Compiling DSL tyto... \n");

    match parse_dsl(input_code) {
        Ok(ast) => {
            println!("✅ AST generated successfully.");

            match StateGraph::from_ast(&ast) {
                Ok(graph) => {
                    println!("✅ Graph generated successfully.");

                    if let Err(errors) = Validator::validate(&graph) {
                        println!("\n❌ Validation Errors Found:");
                        for err in errors {
                            eprintln!("  - {}", err);
                        }
                    } else {
                        println!("✅ Validation passed! The state machine is sound.");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Graph Semantic Error:\n{}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Syntax error:\n{}", e);
        }
    }
}
