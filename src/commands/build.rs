use std::fs;
use std::path::Path;

use crate::compiler::{compile, resolve_output_dir, CompileOptions, CompileTarget};
use crate::config::{GlobalConfig, LocalConfig};
use crate::error::{Error, Result};
use crate::ui;

pub fn run(config_path: &str, machine: Option<&str>) -> Result<()> {
    ui::print_header("Building workspace");

    let global_config =
        GlobalConfig::load(config_path).map_err(|e| Error::config(config_path, e.to_string()))?;

    let workspace_path = Path::new(&global_config.workspace_dir);
    if !workspace_path.exists() || !workspace_path.is_dir() {
        return Err(Error::config(
            config_path,
            format!(
                "Workspace directory '{}' does not exist",
                global_config.workspace_dir
            ),
        ));
    }

    let entries = fs::read_dir(workspace_path).map_err(|e| Error::io(workspace_path, e))?;

    let mut modules_processed = 0;

    for entry in entries {
        let entry = entry.map_err(|e| Error::io(workspace_path, e))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let dir_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        if let Some(target_machine) = machine {
            if target_machine != dir_name {
                continue;
            }
        }

        match build_module(&path, dir_name, &global_config) {
            Ok(()) => modules_processed += 1,
            Err(e) => return Err(e),
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

    Ok(())
}

fn build_module(path: &Path, dir_name: &str, global_config: &GlobalConfig) -> Result<()> {
    ui::module_header(dir_name);

    let local_yaml_path = path.join(format!("{}.yaml", dir_name));

    if !local_yaml_path.exists() {
        ui::warning(&format!(
            "Configuration '{}' not found, skipping",
            local_yaml_path.display()
        ));
        return Ok(());
    }

    let local_config = LocalConfig::load(&local_yaml_path)
        .map_err(|e| Error::config(&local_yaml_path, e.to_string()))?;

    let source_path = path.join(&local_config.source);

    let targets: Vec<CompileTarget> = local_config
        .targets
        .iter()
        .map(|(lang, target_config)| {
            let out_dir = resolve_output_dir(path, Some(&target_config.out_dir), path);
            CompileTarget {
                lang: lang.clone(),
                out_dir,
            }
        })
        .collect();

    let options = CompileOptions {
        source_path,
        targets,
        formatters: global_config.formatters.clone(),
        module_name: Some(dir_name.to_string()),
    };

    let result = compile(options)?;

    for generated in &result.generated {
        ui::success_lang(&generated.lang, &generated.path.display().to_string());
    }

    for warning in &result.warnings {
        ui::warning(warning);
    }

    Ok(())
}
