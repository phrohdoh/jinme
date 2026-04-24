# GitHub Copilot Instructions for jinme

## Project Overview
- `jinme` is a Rust 2024 workspace implementing a Clojure-style reader/interpreter.
- Workspace crates:
  - `crates/jinme`: core interpreter library.
  - `crates/jinme_cli`: CLI/REPL executable.
- Core architecture:
  - `Value` enum models runtime values (scalar, collection, function, var, handle), with optional metadata on every variant.
  - `Environment` + `Namespace` + `Var` provide symbol resolution and runtime bindings.
  - Optics (lenses/prisms) are used for structured, type-safe value access and updates.
- Main libraries:
  - `im` for persistent immutable collections.
  - `nom` for parser combinators.
  - `tracing` + `opentelemetry` for observability.
  - `tokio` for async runtime needs in CLI/tracing setup.

## Coding Standards
- Naming conventions:
  - Rust functions/modules/variables: `snake_case`.
  - Rust structs/enums/traits: `PascalCase`.
  - Rust constants/statics: `UPPER_SNAKE_CASE`.
  - Arc pointer aliases: `Ptr*` (for example `PtrValue`, `PtrVar`).
  - Arc-returning helpers: prefer `_ptr` suffix.
  - Borrowed-view helpers: prefer `_ref` suffix where used.
  - Clojure-visible function names (registered in env): `kebab-case` (for example `get-in`, `assoc-in`, `ns-map`).
- API naming patterns to preserve:
  - Accessors: `get_*`, `try_get_*`.
  - Fallback variants: `*_or`, `*_or_else`, `*_or_nil`, `*_or_panic`.
  - Optics: `preview_*`, `review_*`, `modify_*`, `set_*`, `try_modify_*`.
- Module/file organization:
  - Keep parent modules thin and place focused logic in submodules (for example `float/add.rs`, `value/optics.rs`).
  - Keep optics helpers in `*/optics.rs`; composable helper closures in `*/partials.rs`.
- Linting/formatting requirements:
  - Run `cargo fmt --all` before finalizing changes.
  - Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` for lint-clean code.
  - Run `cargo test --workspace` for regressions.
- Import alias conventions:
  - Use `use crate::prelude::*` for common imports in implementation files.
  - Use explicit module paths for clarity in public API files (for example in `value/partials.rs` we should have `use crate::value::optics` instead of `use super::optics`).
  - Avoid glob imports (`*`) in public API modules to keep exports clear.
  - Maintain consistent aliasing for optics and partials, e.g.: `crate::value::optics as value_optics`, `crate::value::partials as value_partials`, `crate::list::optics as list_optics`, and `crate::list::partials as list_partials`; if they are not consistent, make them so.

## Project Structure
- Root:
  - `Cargo.toml`: workspace manifest.
  - `README.md`: developer usage and examples.
  - `samples/`: sample input programs.
  - `bin/jinme`: helper script for project workflows.
- Core crate (`crates/jinme/src`):
  - Runtime and eval: `core.rs`, `environment.rs`, `namespace.rs`, `var.rs`, `function.rs`.
  - Data model: `value.rs`, `symbol.rs`, `keyword.rs`, `float.rs`, `map.rs`, `set.rs`, `list.rs`, `vector.rs`, `handle.rs`.
  - Parsing: `read.rs` and `read2.rs`.
  - Optics infra: `optics.rs`, `optics/lens.rs`, `optics/prism2.rs`, `optics/prism3.rs`, plus per-type optics modules.
  - Public exports: `lib.rs`, `prelude.rs`.
- CLI crate (`crates/jinme_cli/src/main.rs`):
  - Command entrypoints (`repl`, `eval-string`, `eval-file`, `read-string`, `optics`).
  - Built-in function registration in `create_env()`.
- Tests:
  - Primarily inline unit tests inside each module (`#[cfg(test)] mod tests`).
  - No separate workspace-level integration-test directory is currently used.

## Best Practices
- Security and safety:
  - Avoid `unwrap()`/`expect()` for recoverable failures in new code paths; return typed errors when possible.
  - Never execute external commands or evaluate untrusted input outside established reader/eval flows.
  - Treat handles and mutable shared state (`Mutex`) carefully; avoid holding locks across expensive work.
- Performance:
  - Prefer persistent immutable updates (`assoc`, `dissoc`) and structural sharing over cloning entire collections.
  - Avoid unnecessary `Arc` clones or value materialization in hot paths.
  - Keep parser/eval operations allocation-aware; favor iterator-based transformations.
- Error handling:
  - Reuse existing error types (`ResolveError`, `GetVarError`, `GetValueError`, `GetFunctionError`, reader anomalies).
  - Prefer `Result` propagation with context-rich errors over panics.
  - Keep error conversions explicit via `From` implementations where patterns already exist.
- Metadata and semantics:
  - Preserve metadata when transforming values.
  - Maintain Clojure-like semantics for symbols, vars, namespaces, and collection behavior.
- Observability:
  - Keep `#[tracing::instrument]` on key public operations and preserve structured logging fields.

## Common Tasks
- Adding/changing interpreter behavior:
  - Update core logic in `crates/jinme/src/core.rs` and related runtime modules.
  - If touching value access patterns, update relevant optics/partials modules too.
  - Add/adjust built-ins in CLI `create_env()` with consistent Clojure naming.
- Writing unit tests:
  - Place tests near implementation using `#[cfg(test)]` modules.
  - Follow Arrange/Act/Assert structure and use descriptive snake_case test names.
  - Prefer `assert_eq!` for deterministic value checks.
  - Use `#[should_panic(expected = "...")]` only for intentional panic contracts.
- Documentation updates:
  - Update `README.md` for user-visible behavior, CLI usage, or workflow changes.
  - Add concise `///` docs for non-obvious public APIs and invariants.
  - Keep docs synchronized with command names, module paths, and value semantics.