mod backend;
mod cli;
mod config;
mod frontend;
mod middle;
mod ui;

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
            ui::print_header("Initializing new workspace");

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

            ui::success("Created 'tyto.yaml' (Global Config)");
            ui::success("Created folder 'tyto/example/'");
            ui::success("Created 'tyto/example/example.yaml' (Local Config)");
            ui::success("Created 'tyto/example/example.ty' (DSL Source)");

            ui::complete("Workspace initialized");
            ui::hint("Try running:");
            ui::command_hint("tyto build");
        }
        Commands::Compile {
            source,
            langs,
            out_dir,
        } => {
            ui::print_header("Compiling source file");

            let source_path = Path::new(source);
            let source_dir = source_path.parent().unwrap_or(Path::new("."));
            let file_stem = source_path.file_stem().unwrap().to_str().unwrap();

            let local_yaml_path = source_dir.join(format!("{}.yaml", file_stem));
            let local_config = LocalConfig::load(&local_yaml_path).ok();

            ui::info(&format!("Source: {}", source));

            let source_code = fs::read_to_string(source_path).unwrap_or_else(|_| {
                ui::error(&format!("Source file '{}' not found", source));
                std::process::exit(1);
            });

            let ast = parse_dsl(&source_code).unwrap_or_else(|e| {
                ui::syntax_error(None, &e.to_string());
                std::process::exit(1);
            });

            let graph = StateGraph::from_ast(&ast).expect("Semantic error.");
            Validator::validate(&graph)
                .map_err(|errors| {
                    let err_strings: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
                    ui::validation_errors(None, &err_strings);
                    std::process::exit(1);
                })
                .unwrap();

            let target_langs: Vec<&str> = langs.split(',').collect();

            for lang in target_langs {
                let lang = lang.trim();
                if let Some(generator) = get_generator(lang) {
                    let final_out_dir = if let Some(ref config) = local_config {
                        if let Some(target) = config.targets.get(lang) {
                            let expanded = shellexpand::tilde(&target.out_dir);
                            let out_path = Path::new(expanded.as_ref());
                            if out_path.is_absolute() {
                                out_path.to_path_buf()
                            } else {
                                source_dir.join(out_path)
                            }
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
                    let display_path = file_path.canonicalize().unwrap_or(file_path);
                    ui::success_lang(lang, &display_path.display().to_string());
                } else {
                    ui::warning(&format!("Target language '{}' is not supported", lang));
                }
            }

            ui::complete("Compilation finished");
        }
        Commands::Build { config, machine } => {
            ui::print_header("Building workspace");

            let global_config = match GlobalConfig::load(config) {
                Ok(c) => c,
                Err(e) => {
                    ui::error(&format!("Failed to read config '{}': {}", config, e));
                    std::process::exit(1);
                }
            };

            let workspace_path = Path::new(&global_config.workspace_dir);
            if !workspace_path.exists() || !workspace_path.is_dir() {
                ui::error(&format!(
                    "Workspace directory '{}' does not exist",
                    global_config.workspace_dir
                ));
                std::process::exit(1);
            }

            let mut modules_processed = 0;

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

                    ui::module_header(dir_name);

                    let local_yaml_path = path.join(format!("{}.yaml", dir_name));

                    if !local_yaml_path.exists() {
                        ui::warning(&format!(
                            "Configuration '{}' not found, skipping",
                            local_yaml_path.display()
                        ));
                        continue;
                    }

                    let local_config =
                        LocalConfig::load(&local_yaml_path).expect("Error in local YAML file.");
                    let source_path = path.join(&local_config.source);

                    let source_code = fs::read_to_string(&source_path).unwrap_or_else(|_| {
                        ui::error(&format!(
                            "Source file '{}' not found",
                            source_path.display()
                        ));
                        std::process::exit(1);
                    });

                    let ast = match parse_dsl(&source_code) {
                        Ok(tree) => tree,
                        Err(e) => {
                            ui::syntax_error(Some(dir_name), &e.to_string());
                            std::process::exit(1);
                        }
                    };

                    let graph = StateGraph::from_ast(&ast).expect("Semantic error.");

                    if let Err(errors) = Validator::validate(&graph) {
                        let err_strings: Vec<String> =
                            errors.iter().map(|e| e.to_string()).collect();
                        ui::validation_errors(Some(dir_name), &err_strings);
                        std::process::exit(1);
                    }

                    for (lang, target_config) in &local_config.targets {
                        let expanded = shellexpand::tilde(&target_config.out_dir);
                        let out_path = Path::new(expanded.as_ref());
                        let final_out_dir = if out_path.is_absolute() {
                            out_path.to_path_buf()
                        } else {
                            path.join(out_path)
                        };

                        fs::create_dir_all(&final_out_dir).unwrap();

                        if let Some(generator) = get_generator(lang) {
                            let code = generator.generate(&ast);
                            let file_path = final_out_dir.join(format!(
                                "{}.{}",
                                dir_name,
                                generator.extension()
                            ));

                            fs::write(&file_path, code).expect("Error saving generated file.");
                            let display_path =
                                file_path.canonicalize().unwrap_or(file_path.clone());
                            ui::success_lang(lang, &display_path.display().to_string());

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
                                                ui::warning(&format!(
                                                    "Formatter '{}' failed: {}",
                                                    cmd_str,
                                                    stderr.trim()
                                                ));
                                            }
                                            Err(e) => {
                                                ui::warning(&format!(
                                                    "Could not run formatter '{}': {}",
                                                    cmd_str, e
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            ui::warning(&format!("Target language '{}' is not supported", lang));
                        }
                    }
                    modules_processed += 1;
                }
            }

            if modules_processed == 0 {
                ui::warning("No modules found in workspace");
            } else {
                ui::complete(&format!(
                    "Build finished ({} module{})",
                    modules_processed,
                    if modules_processed == 1 { "" } else { "s" }
                ));
            }
        }
    }
}
