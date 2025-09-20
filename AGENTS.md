# Repository Guidelines

## Project Structure & Module Organization
- `src/lib.rs` exposes `config`, `logging`, and `path`; treat this crate as shared infra for other Galaxy tooling.
- `src/config/` stores configuration IO and lifecycle helpers; keep module tests beside their implementations.
- `src/logging/` centralises flexi_logger setup; add new logging utilities under this directory and re-export selectively.
- `src/path.rs` and `src/types.rs` provide path helpers and shared DTOs consumed by downstream crates.
- Place new feature folders under `src/`, gate unstable APIs behind module-level `pub(crate)` until ready for `lib.rs` re-export.

## Build, Test, and Development Commands
- `cargo build --all` compiles with the default TLS stack; CI runs it twice around TLS toggles, so keep both passes green.
- `cargo test --all -- --test-threads=1` mirrors the workflow matrix and prevents temp-dir races.
- `cargo fmt` applies the required formatting; run `cargo fmt --check` before pushing.
- `cargo clippy -- -D warnings` must succeed locally to keep lint debt out of CI.
- Use `cargo clean` only when moving between TLS builds or diagnosing linker artifacts.

## Coding Style & Naming Conventions
- Rust 2024 edition with standard `rustfmt` defaults (4-space indent, trailing commas, imports grouped by crate).
- Files and modules stay `snake_case`; exported structs/enums use `UpperCamelCase`; traits prefer `PascalCase` suffixes such as `Lifecycle`.
- Return `Result<T, orion_error::Error>` variants from fallible helpers; bubble errors with `?` for clarity.
- Initialise logging through `logging::configure` to ensure flexi_logger picks up the global configuration only once.

## Testing Guidelines
- Co-locate unit tests under `#[cfg(test)]`; add integration suites under `tests/` using filenames like `config_tests.rs`.
- Leverage `rstest` for matrix scenarios and `tempfile::TempDir` for filesystem isolation.
- Cover error paths in backup/restore flows and serialisation round-trips for TOML and YAML inputs.
- Run `cargo test --all` before every PR; attach the command output if CI is pending.

## Commit & Pull Request Guidelines
- Follow the log’s precedent: concise imperative subjects (e.g. `Update orion-error`, `Fix logging init`).
- Include related manifests (`Cargo.toml`, `Cargo.lock`, `version.txt`) in the same commit when bumping versions.
- PR descriptions should outline problem, solution, and validation; mention affected modules by path for reviewers.
- Link issues with `Closes #<id>` and provide screenshots only when behaviour is user-visible (e.g. logging format changes).
