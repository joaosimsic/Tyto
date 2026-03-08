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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("Tyto - Initializing new workspace...");

            let global_yaml = "workspace_dir: \"./tyto\"\nformatters:\n  typescript: \"npx prettier --write\"\n  rust: \"rustfmt\"\n";
            let local_yaml = "source: \"example.ty\"\ntargets:\n  typescript:\n    out_dir: \"../../output/ts\"\n  rust:\n    out_dir: \"../../output/rust\"\n  mermaid:\n    out_dir: \"../../output/docs\"\n";
            let example_ty = "context {\n    user_id: String,\n}\n\nstate Idle {\n    on success Start -> Success;\n}\n\nstate Success {\n    terminal;\n}\n";

            fs::write("tyto.yaml", global_yaml).expect("Failed to create tyto.yaml");

            let example_path = Path::new("tyto/example");
            fs::create_dir_all(example_path).expect("Failed to create tyto/example directory");

            fs::write(example_path.join("example.yaml"), local_yaml)
                .expect("Failed to create local yaml");
            fs::write(example_path.join("example.ty"), example_ty)
                .expect("Failed to create example.ty");

            println!("  [✔] Created 'tyto.yaml' (Global Config)");
            println!("  [✔] Created folder 'tyto/example/'");
            println!("  [✔] Created 'tyto/example/example.yaml' (Local Config)");
            println!("  [✔] Created 'tyto/example/example.ty' (DSL Source)");

            println!("\nWorkspace initialized! Try running:");
            println!("  tyto build");
        }
        Commands::Compile {
            source,
            langs,
            out_dir,
        } => {
            let source_path = Path::new(source);
            let source_dir = source_path.parent().unwrap_or(Path::new("."));
            let file_stem = source_path.file_stem().unwrap().to_str().unwrap();

            let local_yaml_path = source_dir.join(format!("{}.yaml", file_stem));
            let local_config = LocalConfig::load(&local_yaml_path).ok();

            let source_code = fs::read_to_string(source_path).unwrap_or_else(|_| {
                eprintln!("Error: Source file '{}' not found.", source);
                std::process::exit(1);
            });

            let ast = parse_dsl(&source_code).unwrap_or_else(|e| {
                eprintln!("Syntax error:\n{}", e);
                std::process::exit(1);
            });

            let graph = StateGraph::from_ast(&ast).expect("Semantic error.");
            Validator::validate(&graph)
                .map_err(|errors| {
                    eprintln!("Validation Errors:");
                    for err in errors {
                        eprintln!("  - {}", err);
                    }
                    std::process::exit(1);
                })
                .unwrap();

            let target_langs: Vec<&str> = langs.split(',').collect();

            for lang in target_langs {
                let lang = lang.trim();
                if let Some(generator) = get_generator(lang) {
                    let final_out_dir = if let Some(ref config) = local_config {
                        if let Some(target) = config.targets.get(lang) {
                            source_dir.join(&target.out_dir)
                        } else {
                            Path::new(out_dir).to_path_buf()
                        }
                    } else if out_dir != "." {
                        Path::new(out_dir).to_path_buf()
                    } else {
                        source_dir.to_path_buf()
                    };

                    fs::create_dir_all(&final_out_dir).unwrap();

                    let code = generator.generate(&ast);
                    let file_name = format!("{}.{}", file_stem, generator.extension());
                    let file_path = final_out_dir.join(file_name);

                    fs::write(&file_path, code).expect("Error saving file.");
                    println!(
                        "  [{}] successfully generated: {}",
                        lang.to_uppercase(),
                        file_path.display()
                    );
                } else {
                    println!("  Warning: Target language '{}' is not supported.", lang);
                }
            }
        }
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

                            if let Some(formatters) = &global_config.formatters {
                                if let Some(cmd_str) = formatters.get(lang) {
                                    let mut parts = cmd_str.split_whitespace();

                                    if let Some(program) = parts.next() {
                                        let mut command = std::process::Command::new(program);

                                        for arg in parts {
                                            command.arg(arg);
                                        }

                                        command.arg(&file_path);

                                        match command.output() {
                                            Ok(output) if output.status.success() => {}
                                            Ok(output) => {
                                                let stderr =
                                                    String::from_utf8_lossy(&output.stderr);
                                                println!(
                                                    "    Warning: Formatter '{}' failed.\n{}",
                                                    cmd_str, stderr
                                                );
                                            }
                                            Err(e) => {
                                                println!("    Warning: Could not run formatter '{}' ({}). Is it installed in your PATH?", cmd_str, e);
                                            }
                                        }
                                    }
                                }
                            }
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
