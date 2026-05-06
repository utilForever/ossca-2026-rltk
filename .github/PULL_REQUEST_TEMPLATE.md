## What

<!-- One-line summary of what this PR does -->

## Why

<!-- Why is this change needed? Link to issue if applicable -->

## Checklist

### Required

- [ ] `cargo check --all` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple-crate-versions` passes
- [ ] `cargo test --all` passes
- [ ] I linked the related issue (for example: `Closes #123`)

### Functional Validation

- [ ] Behavior related to this change was verified locally (if applicable)
- [ ] Rendering/backend behavior was verified when runtime code changed (if applicable)
- [ ] Algorithm behavior (pathfinding/FOV/noise/random) was verified when affected (if applicable)
- [ ] I added or updated tests for changed behavior (if applicable)

### Configuration & Docs

- [ ] User-facing docs were updated (`README.md`, `ARCHITECTURE.md`, or relevant manual pages, if applicable)
- [ ] New dependencies/configuration are documented (if applicable)
- [ ] No sensitive values or credentials were introduced

### If Applicable

- [ ] Security impact considered (run `cargo audit` locally if needed)
- [ ] Breaking behavior changes are clearly described in this PR
