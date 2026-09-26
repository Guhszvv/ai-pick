# Contributing to ai-pick

Thanks for stopping by. Small CLI, small ruleset.

## Setup

Prerequisites: Rust stable toolchain (edition 2024, so ≥1.85), plus the `fzf` binary for real runs.

```bash
cp config.example.yaml config.yaml   # fill in api_key (gitignored, never commit it)
cargo build
cargo test
```

A full interactive run also needs the models endpoint up at `http://localhost:20128` and the harness binaries (`claude` / `opencode` / `omp`, `ai-jail`, `ai-memory`) — but `cargo test` needs none of that.

## Before opening a PR

Run the same checks CI runs:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo fmt` (not `--check`) fixes formatting for you.

## Conventions

- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/) — `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`. One logical change per commit.
- **PRs:** one feature/fix per PR, green CI, update README/docs when behavior changes.
- **Secrets:** only `sk-test` (tests) and `sk-your-key-here` (example) may appear in diffs. Real keys live in your local `config.yaml`, which is gitignored.
- **Tests:** small test modules stay inline (`#[cfg(test)] mod tests` in the source file); large ones (>100 lines) go in `src/tests/<module>.rs` as submodules via `#[path]`, with access to private items through `super::*`.

## Issues

- Bug reports: include OS, ai-pick version (`git rev-parse --short HEAD` if unreleased), the harness in use (`claude-code` / `opencode` / `omp`), and the exact error output.
- Feature requests: describe the problem first, then the proposed solution and any alternatives you considered.
