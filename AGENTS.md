# AGENTS.md - RCSection

## Project Type

Multi-language: **Rust** (WASM plugin) + **Typst** (frontend/renderer). The Rust crate compiles to `parser.wasm`, which Typst loads as a plugin.

## Architecture

```
Typst raw block (RCS syntax)
    -> WASM plugin (parser.wasm) [CBOR in/out]
    -> Rust parser (LALRPOP grammar)
    -> Rust geometry engine
    -> CBOR-encoded Drawing primitives
    -> Typst/CETZ renderer (draw.typ)
    -> PDF
```

Entrypoints:
- **Rust**: `plugin/src/lib.rs` — WASM exports `priv_parse`, `priv_parse_and_generate`
- **Typst**: `src/rcsection.typ` — imports `parser.wasm`, exposes `parse()`, `rcs-define()`, `init_rcsection()`
- **Renderer**: `src/draw.typ` — consumes `Drawing` primitives from WASM, renders with CETZ

## Critical Workflow

### 1. Always rebuild WASM before testing or committing

The committed `src/parser.wasm` is a build artifact. It goes stale silently.

```bash
just plugin        # cargo build --release --target wasm32-unknown-unknown + cp to src/
```

Run this **before** `cargo test`, `tt run`, or any Typst compilation. The CI does this too.

### 2. Test order

```bash
cargo test         # Rust unit tests (parser + geometry)
just plugin        # Rebuild WASM
tt run             # Typst visual regression tests (tytanic)
```

`just test` runs both `cargo test` and `tt run`.

### 3. Visual regression with tytanic

- Each test is a folder: `tests/<name>/test.typ`
- Reference images live in `tests/<name>/ref/` (not currently in repo — generate with `just update`)
- Generated outputs go to `tests/<name>/out/`
- Diffs go to `tests/<name>/diff/`

```bash
just update        # Generate reference images from current output
```

### 4. Lint gate

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

CI enforces this. LALRPOP generates `grammar.rs` in `target/.../out/` — do not edit it. The generated file may have an unused `FromStr` import; this is normal and suppressed with `#[allow(unused_imports)]` on the `lalrpop_mod!` invocation.

## Style System (Important)

The default drawing style is now **SPD** (technical monochrome). This means:
- All outputs are black-and-white by default
- No colored rebar fills
- Concrete outlines use a heavier stroke
- Stirrups render as hollow rings with double contour

To get the old colorful output:

```rcs
set:
    style "default"
```

To use the technical preset explicitly:

```rcs
set:
    style "spd"
```

## Code Conventions

### Rust (`plugin/`)

- Edition 2024
- Prefer `matches!` over `match` returning `bool`
- Extract helpers for duplicated logic (there are many: `get_section_dims`, `get_cover`, etc.)
- Avoid `.unwrap()` on user-facing data
- Unit tests live in `#[cfg(test)] mod tests` in the same file

### Typst (`src/`)

- Kebab-case for functions: `draw-primitive`, `parse-stroke`
- The `draw.typ` renderer dispatches on `primitive.type` string — keep the enum in sync with Rust

## Gotchas

- **WASM target required**: `rustup target add wasm32-unknown-unknown`
- **Release profile is aggressive**: `lto = true`, `opt-level = 'z'`, `panic = 'abort'` — debug builds are much slower; use `--release`
- **Tytanic not pre-installed**: CI installs `tytanic@0.3.1` via `cache-cargo-install-action`; local dev needs `cargo install tytanic` or equivalent
- **Windows shell in Justfile**: Uses `powershell.exe`, so `&&` and `;` don't work in `just` recipes on Windows
- **No `ref/` images committed yet**: The visual regression suite has `out/` images but no `ref/` baseline. Run `just update` to create them before `tt run` will do meaningful comparisons

## Common Tasks

| Task | Command |
|------|---------|
| Rebuild WASM | `just plugin` |
| Rust tests only | `cargo test` |
| Typst tests only | `tt run` |
| Full test suite | `just test` |
| Update visual refs | `just update` |
| Compile manual | `just doc` |
| Package locally | `just install` |
