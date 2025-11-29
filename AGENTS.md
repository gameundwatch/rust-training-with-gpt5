# Repository Guidelines

## Project Structure & Module Organization
This training crate follows the conventional Cargo layout: `Cargo.toml` and `Cargo.lock` live at the root, shared utilities belong in `src/lib.rs`, and individual exercises or spikes go into `src/bin/<topic>.rs` so they can compile independently. Integration specs go in `tests/`, reusable fixtures in `tests/support`, and any sample inputs or assets should be stored under `assets/` with matching subdirectories for each module.

## Build, Test, and Development Commands
- `cargo check` – Fast type and borrow checker pass; run before every commit.
- `cargo fmt` / `cargo fmt -- --check` – Format code to project defaults and fail CI on drift.
- `cargo clippy --all-targets --all-features` – Lints binaries, library code, and tests; silence warnings only with `#![allow(...)]` plus rationale.
- `cargo test` – Executes unit and integration suites; scope with `cargo test bin_name::` when focusing.
- `cargo run --bin <topic>` – Launches a specific training binary for manual verification.

## Coding Style & Naming Conventions
Rust files use 4-space indentation and must remain rustfmt-clean. Follow idiomatic casing: `snake_case` for functions/modules, `PascalCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants. Keep modules small; move shared helpers into `src/lib.rs` or `src/util/*.rs`. Prefer `Result<T, anyhow::Error>` for application-facing APIs and annotate complex logic with concise `///` doc comments describing intent and invariants.

## Testing Guidelines
Place fast unit tests beside the code inside `#[cfg(test)]` modules and name them after the behavior under test (`handles_empty_payload`). Integration tests belong in `tests/<feature>_spec.rs` and should exercise binaries end-to-end via `assert_cmd`. Mark slow cases with `#[ignore]` and run them via `cargo test -- --ignored`. Strive for 80% line coverage (measure with `cargo llvm-cov --workspace`) and keep fixtures deterministic.

## Commit & Pull Request Guidelines
Use Conventional Commit prefixes (`feat:`, `fix:`, `chore:`, `docs:`) so changelog generation remains reliable. Each commit should bundle a single logical change and include updated docs/tests as needed. Pull requests need a short summary, reproduction or verification notes (commands run, screenshots for CLI output when helpful), and links to any tracking issues. Highlight new binaries or feature flags directly in the PR description to simplify review.
