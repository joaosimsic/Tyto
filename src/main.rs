mod backend;
mod cli;
mod config;
mod frontend;
mod middle;

use clap::Parser;
use std::fs;
use std::path::Path;

use backend::get_generator;
use cli::{Cli, Commands};
use config::{GlobalConfig, LocalConfig};
use frontend::parse_dsl;
use middle::{StateGraph, Validator};

use crate::backend::Generator;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { config, machine } => {
            println!("Tyto - Starting workspace build...\n");

            let global_config = match GlobalConfig::load(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading global configuration '{}': '{}'", config, e);
                    std::process::exit(1);
                }
            };

            let workspace_path = Path::new(&global_config.workspace_dir);
            if !workspace_path.exists() || !workspace_path.is_dir() {
                eprintln!(
                    "The workspace directory '{}' does not exists",
                    global_config.workspace_dir
                );
                std::process::exit(1);
            }

            for entry in fs::read_dir(workspace_path).expect("Error reading workspace_dir") {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().unwrap().to_str().unwrap();

                    if let Some(target_machine) = machine {
                        if target_machine != dir_name {
                            continue;
                        }
                    }

                    println!("Processing module: [{}]", dir_name);

                    let local_yaml_path = path.join(format!("{}.yaml", dir_name));

                    if !local_yaml_path.exists() {
                        println!(
                            "Warning: Configuration '{}' not found. Skipping.",
                            local_yaml_path.display()
                        );
                        continue;
                    }

                    let local_config =
                        LocalConfig::load(&local_yaml_path).expect("Error in local YAML file.");
                    let source_path = path.join(&local_config.source);

                    let source_code = fs::read_to_string(&source_path).unwrap_or_else(|_| {
                        eprintln!(
                            "  Error: Source file '{}' not found.",
                            source_path.display()
                        );
                        std::process::exit(1);
                    });

                    let ast = match parse_dsl(&source_code) {
                        Ok(tree) => tree,
                        Err(e) => {
                            eprintln!("  Syntax error in the DSL module '{}':\n{}", dir_name, e);
                            std::process::exit(1);
                        }
                    };

                    let graph = StateGraph::from_ast(&ast).expect("  Semantic error.");

                    if let Err(errors) = Validator::validate(&graph) {
                        eprintln!("  Validation Errors in '{}':", dir_name);
                        for err in errors {
                            eprintln!("    - {}", err);
                        }
                        std::process::exit(1);
                    }

                    for (lang, target_config) in &local_config.targets {
                        fs::create_dir_all(&target_config.out_dir).unwrap();

                        if let Some(generator) = get_generator(lang) {
                            let code = generator.generate(&ast);
                            let file_path = std::path::Path::new(&target_config.out_dir)
                                .join(format!("{}.{}", dir_name, generator.extension()));

                            fs::write(&file_path, code).expect("Error to save generated file.");
                            println!(
                                "  [{}] successfully generated in: {}",
                                lang.to_uppercase(),
                                file_path.display()
                            );
                        } else {
                            println!(
                                "  Warning: Target language '{}' is not supported by Tyto.",
                                lang
                            );
                        }
                    }
                    println!("");
                }
            }
            println!("Workspace build complete!");
        }
    }
}
