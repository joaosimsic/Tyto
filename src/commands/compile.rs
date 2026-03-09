use std::path::Path;

use crate::compiler::{compile, resolve_output_dir, CompileOptions, CompileTarget};
use crate::config::LocalConfig;
use crate::error::Result;
use crate::ui;

pub fn run(source: &str, langs: &str, out_dir: &str) -> Result<()> {
    ui::print_header("Compiling source file");

    let source_path = Path::new(source);
    let source_dir = source_path.parent().unwrap_or(Path::new("."));
    let file_stem = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    ui::info(&format!("Source: {}", source));

    let local_yaml_path = source_dir.join(format!("{}.yaml", file_stem));
    let local_config = LocalConfig::load(&local_yaml_path).ok();

    let target_langs: Vec<&str> = langs.split(',').map(|s| s.trim()).collect();

    let targets: Vec<CompileTarget> = target_langs
        .iter()
        .map(|lang| {
            let resolved_dir = if let Some(ref config) = local_config {
                if let Some(target) = config.targets.get(*lang) {
                    resolve_output_dir(source_dir, Some(&target.out_dir), Path::new(out_dir))
                } else {
                    resolve_output_dir(source_dir, None, Path::new(out_dir))
                }
            } else if out_dir != "." {
                Path::new(out_dir).to_path_buf()
            } else {
                source_dir.to_path_buf()
            };

            CompileTarget {
                lang: lang.to_string(),
                out_dir: resolved_dir,
            }
        })
        .collect();

    let options = CompileOptions {
        source_path: source_path.to_path_buf(),
        targets,
        formatters: None,
        module_name: None,
    };

    let result = compile(options)?;

    for generated in &result.generated {
        ui::success_lang(&generated.lang, &generated.path.display().to_string());
    }

    for warning in &result.warnings {
        ui::warning(warning);
    }

    ui::complete("Compilation finished");

    Ok(())
}
