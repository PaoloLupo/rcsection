root := justfile_directory()

plugin:
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/parser.wasm src/