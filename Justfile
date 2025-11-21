set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

root := justfile_directory()
export TYPST_ROOT := root
python := if os() == "macos" { "python3" } else if os() == "windows" { "py" } else { "python" }

[private]
default:
    @just --list --unsorted

# Generate documentation
doc:
    typst compile docs/manual.typ docs/manual.pdf

# Run test suite
test *args:
    cargo test
    tt run {{ args }}

# Generate tests from examples
gen-test:
    {{ python }} ./scripts/gen_test.py

# Update test cases (change previous images references)
update *args:
    tt update {{ args }}

# Package the library into the specified directory
package target:
    {{ python }} ./scripts/package.py {{ target }}

# Install the library with the "@local" prefix
install: (package "@local")

# Install the library with the "@preview" prefix
install-preview: (package "@preview")

[private]
remove target:
    {{ python }} ./scripts/uninstall.py {{ target }}

# Uninstalls the library with the "@local" prefix
uninstall: (remove "@local")

# Uninstalls the library with the "@local" prefix
uninstall-preview: (remove "@preview")

# Build the wasm plugin and copy in src directory
plugin:
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/parser.wasm src/

# run test suite and documentation
ci: test doc
