mod backend;
mod cli;
mod commands;
mod compiler;
mod config;
mod error;
mod frontend;
mod middle;
mod templates;
mod ui;

use clap::Parser;

use cli::{Cli, Commands};
use error::Error;

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init => commands::init(),
        Commands::Compile {
            source,
            langs,
            out_dir,
        } => commands::compile(source, langs, out_dir),
        Commands::Build { config, machine } => commands::build(config, machine.as_deref()),
    };

    if let Err(e) = result {
        handle_error(e);
        std::process::exit(1);
    }
}

fn handle_error(error: Error) {
    match error {
        Error::Validation { module, errors } => {
            ui::validation_errors(module.as_deref(), &errors);
        }
        Error::Parse { source, message } => {
            ui::syntax_error(Some(&source), &message);
        }
        _ => {
            ui::error(&error.to_string());
        }
    }
}
