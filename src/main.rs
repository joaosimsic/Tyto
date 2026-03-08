mod frontend;
mod middle;

use frontend::parse_dsl;

fn main() {
    let input_code = r#"
        state Pendente {
            on Pagar -> Pago;
            on Cancelar -> Cancelado;
        }

        state Pago {
            data { transaction_id: String, }
            on Enviar -> Enviado;
            on Reembolsar -> Reembolsado;
        }

        state Enviado {
            terminal; 
        }
    "#;

    println!("Compiling DSL tyto... \n");

    match parse_dsl(input_code) {
        Ok(ast) => {
            println!("AST generated");
            println!("{:#?}", ast);
        }
        Err(e) => {
            eprintln!("Syntax error:\n{}", e);
        }
    }
}
