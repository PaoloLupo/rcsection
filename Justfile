root := justfile_directory()

export TYPST_ROOT := root

python := if os() == "macos" {
    "python3"
} else {
    "python"
}

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

plugin:
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/parser.wasm src/

# run ci suite
ci: test doc
