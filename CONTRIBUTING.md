# Contributing to ossca-2026-rltk

Thanks for contributing to **ossca-2026-rltk**. This guide explains the expected workflow and quality bar for pull requests.

## Getting Started

1. Install the stable Rust toolchain.
2. Clone the repository and open it in your terminal.
3. Run the core checks before opening a PR:

```sh
cargo check --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions
cargo test --all
```

Optional local build:

```sh
cargo build
```

Optional dependency checks:

```sh
cargo +nightly udeps --all-targets
cargo audit
```

## Architecture

`ossca-2026-rltk` is a Rust workspace centered on a stable facade crate (`bracket-lib`) with focused `bracket-*` crates and a compatibility facade (`rltk`).

- `bracket-lib` and `rltk` are the primary high-level entry points for users.
- Runtime and rendering backends are in `bracket-terminal` (feature-gated by backend).
- Algorithms and shared abstractions live in `bracket-pathfinding` and `bracket-algorithm-traits`.
- Supporting domains are split into focused crates such as `bracket-color`, `bracket-geometry`, `bracket-random`, and `bracket-noise`.

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full crate map, runtime flow, and extension boundaries.

## Code Style

- Run `cargo fmt --all` before committing.
- Treat Clippy warnings as errors:

```sh
cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions
```

- Keep crate responsibilities focused and avoid leaking implementation-only internals across crate boundaries.
- Prefer feature-gated backend integrations instead of runtime branching for backend selection.
- Use additive public API changes; avoid breaking downstream users unnecessarily.
- Keep changes focused and avoid unrelated refactors.
- Add or update tests when behavior changes.

## Pull Requests

- Open pull requests against the `main` branch.
- Keep each PR focused on one meaningful change.
- Link the related issue in the PR description (for example: `Closes #123`).
- Ensure CI-equivalent checks pass locally:

```sh
cargo check --all
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions
cargo test --all
```

- Update user-facing docs when behavior or usage changes.
- Do not include sensitive values or credentials.

## Releasing

The project follows Conventional Commit-style release commit conventions:

- Release commit format: `feat: vX.Y.Z — short summary`
- Hotfix format: `fix: description` (no version in the message)

Before preparing a release change, make sure all CI jobs pass for the release-related updates.

## Dependencies

Dependency and ecosystem checks are part of contributor quality expectations:

- Prefer minimal, well-maintained crates.
- Document new dependencies or configuration changes in the PR.
- When dependencies change, run:

```sh
cargo +nightly udeps --all-targets
cargo audit
```
