use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use tyto::compiler::{compile, CompileOptions, CompileTarget};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn temp_dir(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("tyto-test-{}-{}", tag, nanos));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn compile_all(name: &str, out: &Path) -> Vec<PathBuf> {
    let langs = ["typescript", "rust", "java", "mermaid"];
    let targets = langs
        .iter()
        .map(|lang| CompileTarget {
            lang: lang.to_string(),
            out_dir: out.to_path_buf(),
        })
        .collect();

    let result = compile(CompileOptions {
        source_path: fixture(name),
        targets,
        formatters: None,
        module_name: None,
    })
    .expect("compilation must succeed");

    assert_eq!(result.generated.len(), langs.len());
    assert!(result.warnings.is_empty());

    result.generated.into_iter().map(|g| g.path).collect()
}

fn tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn assert_contains(path: &Path, needle: &str) {
    let contents = std::fs::read_to_string(path).unwrap();
    assert!(
        contents.contains(needle),
        "expected {:?} to contain {:?}",
        path,
        needle
    );
}

#[test]
fn generates_all_targets_for_order() {
    let out = temp_dir("order");
    let files = compile_all("order.ty", &out);

    let stem = "order";
    assert!(files.iter().any(|p| p.ends_with(format!("{}.tyto.rs", stem))));
    assert!(files.iter().any(|p| p.ends_with(format!("{}.tyto.ts", stem))));
    assert!(files.iter().any(|p| p.ends_with(format!("{}.java", stem))));
    assert!(files.iter().any(|p| p.ends_with(format!("{}.mmd", stem))));

    for file in &files {
        let len = std::fs::metadata(file).unwrap().len();
        assert!(len > 0, "{:?} must not be empty", file);
    }

    assert_contains(&out.join("order.tyto.rs"), "pub struct PendingState");
    assert_contains(&out.join("order.tyto.ts"), "PendingState");
    assert_contains(&out.join("order.java"), "sealed interface TytoState");
    assert_contains(&out.join("order.mmd"), "stateDiagram-v2");

    std::fs::remove_dir_all(&out).ok();
}

#[test]
fn generated_rust_compiles_with_rustc() {
    let out = temp_dir("rustc");
    compile_all("order.ty", &out);
    let source = out.join("order.tyto.rs");
    let artifact = out.join("order.tyto.rlib");

    let status = Command::new("rustc")
        .args([
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            "--crate-name",
            "tyto_order",
            "--emit=metadata",
        ])
        .arg("-o")
        .arg(&artifact)
        .arg(&source)
        .status()
        .expect("rustc must be available");

    assert!(status.success(), "generated Rust must compile");
    std::fs::remove_dir_all(&out).ok();
}

#[test]
fn generated_typescript_compiles_with_tsc() {
    if !tool_available("tsc") {
        eprintln!("skipping: tsc not installed");
        return;
    }
    let out = temp_dir("tsc");
    compile_all("order.ty", &out);
    let source = out.join("order.tyto.ts");

    let status = Command::new("tsc")
        .args(["--noEmit", "--skipLibCheck", "--strict"])
        .arg(&source)
        .status()
        .expect("tsc must run");

    assert!(status.success(), "generated TypeScript must compile");
    std::fs::remove_dir_all(&out).ok();
}

#[test]
fn generated_java_compiles_with_javac() {
    if !tool_available("javac") {
        eprintln!("skipping: javac not installed");
        return;
    }
    let out = temp_dir("javac");
    compile_all("order.ty", &out);
    let source = out.join("order.java");

    let status = Command::new("javac")
        .arg("-d")
        .arg(&out)
        .arg(&source)
        .status()
        .expect("javac must run");

    assert!(status.success(), "generated Java must compile");
    std::fs::remove_dir_all(&out).ok();
}
