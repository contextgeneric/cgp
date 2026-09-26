//! Turns the wiring example in each error backend's README into a test module, so the
//! `error_backends` target compiles and runs the README's own code instead of a copy of it.
//!
//! Each README carries one `rust,ignore` block: `use` lines and items, then top-level statements
//! after the last item's closing brace. The block is split there, and the statements become the
//! body of a `#[test]` function, written to `$OUT_DIR/readme_<backend>.rs`.

use std::path::Path;
use std::{env, fs};

const BACKENDS: [&str; 3] = ["anyhow", "eyre", "std"];

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    for backend in BACKENDS {
        let readme = Path::new(&manifest_dir)
            .join("../../standalone/error")
            .join(format!("cgp-error-{backend}"))
            .join("README.md");
        println!("cargo::rerun-if-changed={}", readme.display());

        let text = fs::read_to_string(&readme)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", readme.display()));
        let module =
            readme_test(backend, &text).unwrap_or_else(|e| panic!("{}: {e}", readme.display()));

        fs::write(
            Path::new(&out_dir).join(format!("readme_{backend}.rs")),
            module,
        )
        .unwrap();
    }
}

fn readme_test(backend: &str, readme: &str) -> Result<String, &'static str> {
    let block = readme
        .split("```rust,ignore\n")
        .nth(1)
        .and_then(|rest| rest.split("\n```").next())
        .ok_or("no `rust,ignore` block")?;

    let lines: Vec<&str> = block.lines().collect();
    let split = lines
        .iter()
        .rposition(|line| line.starts_with('}'))
        .ok_or("the block has no top-level item")?;

    let items = lines[..=split].join("\n");
    let statements: Vec<String> = lines[split + 1..]
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| format!("    {line}"))
        .collect();
    if statements.is_empty() {
        return Err("the block has no statements after its items");
    }

    Ok(format!(
        "{items}\n\n#[test]\nfn test_readme_{backend}() {{\n{}\n}}\n",
        statements.join("\n")
    ))
}
