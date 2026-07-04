//! Workspace invariant audits (refactor plan Phase 7 / ses-core-build-plan §9).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::Path;
use std::process::Command;

const CRATE_ROOTS: &[&str] = &[
    concat!(env!("CARGO_MANIFEST_DIR"), "/../ses-core"),
    concat!(env!("CARGO_MANIFEST_DIR"), "/../ses-engineer"),
];

const LINE_LIMIT: usize = 120;

const LINE_ALLOWLIST: &[&str] = &[
    "dimtype/markers.rs",
    "dimtype/table_mul.rs",
    "dimtype/table_div.rs",
    "dimtype/table_identity.rs",
    "tower/newtypes.rs",
    "expr/ast.rs",
    "repr_error.rs",
    "quantity/from_authored.rs",
];

const MULTI_FN_ALLOWLIST: &[&str] = &["policy/policies.rs", "provenance/stale_pins.rs", "lib.rs"];

const DOC_ALLOWLIST: &[&str] = &["lib.rs", "dimtype/markers.rs", "expr/ast.rs"];

const DOC_SKIP_CRATES: &[&str] = &["ses-core"];

#[test]
fn zero_runtime_dependencies_ses_core() {
    let output = cargo_tree(&["-e", "normal", "-p", "ses-core"]);
    let tree = String::from_utf8(output.stdout).expect("utf8");
    let deps: Vec<&str> = tree
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    assert!(
        deps.is_empty(),
        "ses-core must have zero normal dependencies:\n{tree}"
    );
}

#[test]
fn ses_engineer_exact_dependency_set() {
    let output = cargo_tree(&["-e", "normal", "-p", "ses-engineer", "--depth", "1"]);
    let tree = String::from_utf8(output.stdout).expect("utf8");
    let deps: Vec<String> = tree
        .lines()
        .skip(1)
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("├── ")
                .or_else(|| trimmed.strip_prefix("└── "))
                .map(|rest| rest.split_whitespace().next().unwrap_or("").to_string())
        })
        .collect();
    let mut sorted = deps;
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        sorted,
        ["adele-ring", "serde", "ses-core", "thiserror"],
        "unexpected ses-engineer dependencies:\n{tree}"
    );
}

#[test]
fn no_float_types_in_constitution_and_engineer_src() {
    for root in CRATE_ROOTS {
        walk_no_floats(&Path::new(root).join("src"), &[]).expect("float audit");
    }
}

#[test]
fn clippy_disallows_float_types() {
    for root in CRATE_ROOTS {
        let path = Path::new(root).join("clippy.toml");
        let src = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing clippy.toml at {}", path.display()));
        assert!(
            src.contains("f32") && src.contains("f64"),
            "clippy.toml must disallow f32/f64: {}",
            path.display()
        );
    }
}

#[test]
fn no_mod_rs_files() {
    for root in CRATE_ROOTS {
        walk_no_mod_rs(&Path::new(root).join("src")).expect("mod.rs audit");
    }
}

#[test]
fn file_line_limit() {
    for root in CRATE_ROOTS {
        walk_line_limit(&Path::new(root).join("src"), root).expect("line limit");
    }
}

#[test]
fn at_most_one_pub_fn_per_file() {
    for root in CRATE_ROOTS {
        walk_pub_fn_limit(&Path::new(root).join("src"), root).expect("pub fn limit");
    }
}

#[test]
fn public_items_have_doc_comments() {
    for root in CRATE_ROOTS {
        let name = Path::new(root).file_name().unwrap().to_string_lossy();
        if DOC_SKIP_CRATES.iter().any(|c| *c == name) {
            continue;
        }
        walk_doc_coverage(&Path::new(root).join("src"), root).expect("doc coverage");
    }
}

#[test]
fn policies_inventory_complete() {
    let entries = ses_core::policies();
    assert_eq!(entries.len(), 4);
    assert!(entries.iter().any(|e| e.name.contains("no-floats")));
}

fn cargo_tree(args: &[&str]) -> std::process::Output {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("Cargo.toml");
    Command::new("cargo")
        .arg("tree")
        .args(args)
        .arg("--manifest-path")
        .arg(&manifest)
        .output()
        .expect("cargo tree")
}

fn strip_test_cfg_blocks(src: &str) -> String {
    let mut out = String::new();
    let mut in_test_block = false;
    for line in src.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[cfg(test)]") {
            in_test_block = true;
            continue;
        }
        if in_test_block {
            if trimmed == "}" && !line.starts_with(' ') && !line.starts_with('\t') {
                in_test_block = false;
            }
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn rel_path(full: &Path, crate_root: &str) -> String {
    full.strip_prefix(Path::new(crate_root).join("src"))
        .unwrap_or(full)
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_allowlisted(rel: &str, allowlist: &[&str]) -> bool {
    allowlist.iter().any(|suffix| rel.ends_with(suffix))
}

fn walk_no_floats(dir: &Path, errors: &[String]) -> Result<(), String> {
    let mut errors = errors.to_vec();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_no_floats(&path, &errors)?;
            continue;
        }
        if path.extension().is_some_and(|e| e == "rs") {
            let src = strip_test_cfg_blocks(&fs::read_to_string(&path).map_err(|e| e.to_string())?);
            if src.contains("f32") || src.contains("f64") {
                errors.push(format!("float type in {}", path.display()));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn walk_no_mod_rs(dir: &Path) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_no_mod_rs(&path)?;
        } else if path.file_name().is_some_and(|n| n == "mod.rs") {
            return Err(format!("mod.rs forbidden: {}", path.display()));
        }
    }
    Ok(())
}

fn walk_line_limit(dir: &Path, crate_root: &str) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_line_limit(&path, crate_root)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let rel = rel_path(&path, crate_root);
        if is_allowlisted(&rel, LINE_ALLOWLIST) {
            continue;
        }
        let src = strip_test_cfg_blocks(&fs::read_to_string(&path).map_err(|e| e.to_string())?);
        let lines = src.lines().filter(|l| !l.trim().is_empty()).count();
        if lines > LINE_LIMIT {
            return Err(format!(
                "{rel} has {lines} non-empty lines (limit {LINE_LIMIT})"
            ));
        }
    }
    Ok(())
}

fn walk_pub_fn_limit(dir: &Path, crate_root: &str) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_pub_fn_limit(&path, crate_root)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let rel = rel_path(&path, crate_root);
        if is_allowlisted(&rel, MULTI_FN_ALLOWLIST) {
            continue;
        }
        let src = strip_test_cfg_blocks(&fs::read_to_string(&path).map_err(|e| e.to_string())?);
        let pub_fns = src
            .lines()
            .filter(|line| line.starts_with("pub fn ") || line.starts_with("pub async fn "))
            .count();
        if pub_fns > 1 {
            return Err(format!("{rel} has {pub_fns} pub fn items (limit 1)"));
        }
    }
    Ok(())
}

fn walk_doc_coverage(dir: &Path, crate_root: &str) -> Result<(), String> {
    let mut errors = Vec::new();
    walk_doc_coverage_inner(dir, crate_root, &mut errors)?;
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn walk_doc_coverage_inner(
    dir: &Path,
    crate_root: &str,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_doc_coverage_inner(&path, crate_root, errors)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let rel = rel_path(&path, crate_root);
        if is_allowlisted(&rel, DOC_ALLOWLIST) {
            continue;
        }
        let src = strip_test_cfg_blocks(&fs::read_to_string(&path).map_err(|e| e.to_string())?);
        let lines: Vec<&str> = src.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if is_public_item_line(line) && !has_doc_comment(&lines, idx) {
                errors.push(format!("{rel}:{} missing doc on `{line}`", idx + 1));
            }
        }
    }
    Ok(())
}

fn is_public_item_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("pub struct ")
        || trimmed.starts_with("pub enum ")
        || trimmed.starts_with("pub trait ")
        || trimmed.starts_with("pub fn ")
        || trimmed.starts_with("pub type ")
        || trimmed.starts_with("pub const ")
}

fn has_doc_comment(lines: &[&str], item_idx: usize) -> bool {
    for line in lines[..item_idx].iter().rev() {
        let trimmed = line.trim();
        if is_public_item_line(trimmed) {
            return false;
        }
        if trimmed.starts_with("///") {
            return true;
        }
    }
    false
}
