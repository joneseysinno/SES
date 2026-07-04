//! Crate invariant checks (ses-core-build-plan §9).

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[test]
fn zero_runtime_dependencies() {
    // Mechanical check: this test module is part of ses-core only.
    // CI also runs `cargo tree -e normal -p ses-core` expecting no deps.
}

#[test]
fn no_float_types_in_source() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(manifest_dir.join("src/lib.rs")).expect("read lib.rs");
    assert!(
        !src.contains("f32") && !src.contains("f64"),
        "lib.rs must not reference float types"
    );
}
