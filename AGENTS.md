# AGENTS.md

This file provides context and instructions for AI coding agents working on the **ossca-2026-rltk** project.

## Project Overview

**ossca-2026-rltk** is a Rust workspace based on the `bracket-lib` ecosystem. It includes a facade crate (`bracket-lib`), focused `bracket-*` crates for rendering/algorithms/utilities, and a compatibility facade (`rltk`).

The repository is used in the 2026 OSSCA contribution program and emphasizes clear crate boundaries, feature-gated backends, and contributor-friendly documentation.

## Tech Stack

| Component  | Details                                       |
| ---------- | --------------------------------------------- |
| Language   | Rust (edition 2021)                           |
| Build tool | Cargo                                         |
| Toolchain  | stable (plus nightly for `cargo udeps` in CI) |

## Repository Structure

```text
ossca-2026-rltk/
├── src/                         # bracket-lib facade exports
├── bracket-algorithm-traits/    # shared algorithm abstractions
├── bracket-color/               # color types and transforms
├── bracket-geometry/            # geometry primitives and helpers
├── bracket-noise/               # noise generation utilities
├── bracket-pathfinding/         # pathfinding and FOV implementations
├── bracket-random/              # RNG and dice parsing utilities
├── bracket-terminal/            # runtime loop and rendering backends
├── bracket-rex/                 # RexPaint support
├── bracket-embedding/           # embedding/linking helpers
├── bracket-bevy/                # Bevy integration crate
├── rltk/                        # backward-compatible facade
├── manual/                      # mdBook source
├── .github/workflows/rust.yml   # CI pipeline
├── ARCHITECTURE.md              # architecture and crate map
└── README.md
```

## Architecture Conventions

- Keep `bracket-lib` and `rltk` stable as high-level facades for downstream users.
- Keep crate responsibilities focused (runtime in `bracket-terminal`, algorithms in `bracket-pathfinding`, shared traits in `bracket-algorithm-traits`, etc.).
- Prefer feature-gated backend integrations instead of runtime branching for platform/rendering selection.
- Use additive changes to public APIs; avoid leaking implementation-only internals.
- Follow existing crate/module organization before introducing new shared helpers or cross-crate dependencies.

## Common Commands

### Build & Check

```sh
cargo build
cargo check --all
```

### Test

```sh
cargo test --all
```

### Lint & Format

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions
```

### Dependency & Security Checks

```sh
cargo +nightly udeps --all-targets
cargo audit
```

## Commit Style

- Conventional Commits: `feat:`, `fix:`, `refactor:`, `perf:`, `test:`, `docs:`, `chore:`
- Split commits by behavior or another meaningful unit of change.
- Release commits: `feat: vX.Y.Z — short summary`
- Hotfix: `fix: description` (no version in message)

## CI Pipeline

The GitHub Actions workflow (`.github/workflows/rust.yml`) runs on push/PR to `main`:

1. **Check**: `cargo check --all`
2. **Lint**: `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions`
3. **Test**: `cargo test --all`
4. **Unused Crates**: `cargo +nightly udeps --all-targets`
5. **Audit**: dependency security audit via `actions-rust-lang/audit`

All CI jobs must pass before merging a pull request.

## Contribution Guidelines

- Keep code formatted with `cargo fmt` before committing.
- Fix all `cargo clippy` warnings — the CI enforces `-D warnings`.
- Add tests for new functionality in the relevant module; for split domains, prefer colocated `tests.rs`.
- Keep commits focused and write clear commit messages.
- Open a pull request targeting the `main` branch.
