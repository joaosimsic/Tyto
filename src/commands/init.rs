use std::fs;
use std::path::Path;

use crate::error::{Error, Result};
use crate::templates;
use crate::ui;

pub fn run() -> Result<()> {
    ui::print_header("Initializing new workspace");

    fs::write("tyto.yaml", templates::GLOBAL_CONFIG).map_err(|e| Error::io("tyto.yaml", e))?;
    ui::success("Created 'tyto.yaml' (Global Config)");

    let example_path = Path::new("tyto/example");
    fs::create_dir_all(example_path).map_err(|e| Error::io(example_path, e))?;
    ui::success("Created folder 'tyto/example/'");

    let local_yaml = example_path.join("example.yaml");
    fs::write(&local_yaml, templates::LOCAL_CONFIG).map_err(|e| Error::io(&local_yaml, e))?;
    ui::success("Created 'tyto/example/example.yaml' (Local Config)");

    let example_ty = example_path.join("example.ty");
    fs::write(&example_ty, templates::EXAMPLE_SOURCE).map_err(|e| Error::io(&example_ty, e))?;
    ui::success("Created 'tyto/example/example.ty' (DSL Source)");

    ui::complete("Workspace initialized");
    ui::hint("Try running:");
    ui::command_hint("tyto build");

    Ok(())
}
