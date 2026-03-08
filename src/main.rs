mod frontend; 
mod middle; 
mod backend;

use frontend::parse_dsl; 
use middle::{StateGraph, Validator}; 
use backend::generate_ts;

fn main() {
    let input_code = r#"
        state Pendente {
            data { id: String, }
            on Pagar -> Pago;
        }

        state Pago {
            data { transaction_id: String, amount: f64, }
            on Enviar -> Enviado;
        }

        state Enviado {
            terminal; 
        }
    "#;

    println!("Compilando DSL Tyto... \n");

    match parse_dsl(input_code) {
        Ok(ast) => {
            println!("✅ AST gerada com sucesso.");

            match StateGraph::from_ast(&ast) { 
                Ok(graph) => {
                    if let Err(errors) = Validator::validate(&graph) { 
                        println!("\n❌ Erros de Validação Encontrados:");
                        for err in errors {
                            eprintln!("  - {}", err);
                        }
                    } else {
                        println!("✅ Validação semântica aprovada!");
                        
                        let ts_code = generate_ts(&ast);
                        println!("\n📦 Código TypeScript Gerado:\n");
                        println!("{}", ts_code);
                    }
                }
                Err(e) => eprintln!("❌ Erro Semântico no Grafo:\n{}", e),
            }
        }
        Err(e) => eprintln!("❌ Erro de sintaxe:\n{}", e), //
    }
}
