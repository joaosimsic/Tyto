use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::backend::get_generator;
use crate::error::{Error, Result};
use crate::frontend::ast::TytoProgram;
use crate::frontend::parse_dsl;
use crate::middle::{StateGraph, Validator};

pub struct CompileTarget {
    pub lang: String,
    pub out_dir: PathBuf,
}

pub struct CompileOptions {
    pub source_path: PathBuf,
    pub targets: Vec<CompileTarget>,
    pub formatters: Option<HashMap<String, String>>,
    pub module_name: Option<String>,
}

pub struct CompileResult {
    pub generated: Vec<GeneratedFile>,
    pub warnings: Vec<String>,
}

pub struct GeneratedFile {
    pub lang: String,
    pub path: PathBuf,
}

pub fn parse_and_validate(source_path: &Path, module_name: Option<&str>) -> Result<TytoProgram> {
    let source_code = fs::read_to_string(source_path).map_err(|e| Error::io(source_path, e))?;

    let ast = parse_dsl(&source_code)
        .map_err(|e| Error::parse(source_path.display().to_string(), e.to_string()))?;

    let graph = StateGraph::from_ast(&ast).map_err(|e| Error::semantic(e))?;

    Validator::validate(&graph)
        .map_err(|errors| Error::validation(module_name.map(String::from), errors))?;

    Ok(ast)
}

pub fn compile(options: CompileOptions) -> Result<CompileResult> {
    let module_name = options.module_name.as_deref();
    let ast = parse_and_validate(&options.source_path, module_name)?;

    let file_stem = options
        .source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let mut result = CompileResult {
        generated: Vec::new(),
        warnings: Vec::new(),
    };

    for target in &options.targets {
        match generate_for_target(&ast, file_stem, target, &options.formatters) {
            Ok(generated) => result.generated.push(generated),
            Err(Error::UnsupportedLanguage { lang }) => {
                result
                    .warnings
                    .push(format!("Target language '{}' is not supported", lang));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}

fn generate_for_target(
    ast: &TytoProgram,
    file_stem: &str,
    target: &CompileTarget,
    formatters: &Option<HashMap<String, String>>,
) -> Result<GeneratedFile> {
    let generator =
        get_generator(&target.lang).ok_or_else(|| Error::unsupported_language(&target.lang))?;

    fs::create_dir_all(&target.out_dir).map_err(|e| Error::io(&target.out_dir, e))?;

    let code = generator.generate(ast);
    let file_name = format!("{}.{}", file_stem, generator.extension());
    let file_path = target.out_dir.join(&file_name);

    fs::write(&file_path, code).map_err(|e| Error::io(&file_path, e))?;

    if let Some(formatters) = formatters {
        if let Some(cmd_str) = formatters.get(&target.lang) {
            run_formatter(cmd_str, &file_path);
        }
    }

    let display_path = file_path.canonicalize().unwrap_or(file_path);

    Ok(GeneratedFile {
        lang: target.lang.clone(),
        path: display_path,
    })
}

fn run_formatter(cmd_str: &str, file_path: &Path) {
    let mut parts = cmd_str.split_whitespace();
    let Some(program) = parts.next() else {
        return;
    };

    let mut command = Command::new(program);
    for arg in parts {
        command.arg(arg);
    }
    command.arg(file_path);

    let _ = command.output();
}

pub fn resolve_output_dir(
    base_dir: &Path,
    config_out_dir: Option<&str>,
    fallback: &Path,
) -> PathBuf {
    match config_out_dir {
        Some(dir) => {
            let expanded = shellexpand::tilde(dir);
            let out_path = Path::new(expanded.as_ref());
            if out_path.is_absolute() {
                out_path.to_path_buf()
            } else {
                base_dir.join(out_path)
            }
        }
        None => fallback.to_path_buf(),
    }
}
