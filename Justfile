root := justfile_directory()

export TYPST_ROOT := root

[private]
default:
    @just --list --unsorted

# Run test suite
test *args:
    # cargo test
    tt run {{ args }}

# Update test cases (change previous images references)
update *args:
    tt update {{ args }}

# Package the library into the specified directory
package target:
    python3 ./scripts/package.py {{ target }}

# Install the library with the "@local" prefix
install: (package "@local")

# Install the library with the "@preview" prefix
install-preview: (package "@preview")

[private]
remove target:
    python3 ./scripts/uninstall.py {{ target }}

# Uninstalls the library with the "@local" prefix
uninstall: (remove "@local")

# Uninstalls the library with the "@local" prefix
uninstall-preview: (remove "@preview")

plugin:
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/parser.wasm src/
