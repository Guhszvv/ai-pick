# AGENTS.md — ai-pick

Rust CLI that picks a model via `fzf` and `exec`s the harness with `--model`.

## Commands

- `cargo test` — all unit tests (inline `mod tests` per file). Single test: `cargo test <name>`.
- `cargo build` — binary at `./target/debug/ai-pick`. `--help` works without config: `./target/debug/ai-pick --help`.
- Full run needs: `config.yaml` present + `fzf` installed + models endpoint up at `http://localhost:20128`.

## Setup / runtime deps (all required for a real run)

- `cp config.example.yaml config.yaml` and fill `api_key`. `config.yaml` is gitignored — never commit it.
- External `fzf` binary is spawned by `src/fzf.rs`; missing binary = runtime error, not a compile error.
- `src/api.rs` hardcodes `http://localhost:20128/v1/models` — `anthropic_base_url` from config is only used for the exec env, not for fetching models.

## Architecture (src/)

- `main.rs`: `Cli::parse` → `Config::load` → `get_models` → `select_model` → maybe `Config::save` → `CommandExt::exec` (replaces process; `exec` never returns on success).
- `cli.rs`: `--claude-code` / `--opencode` (conflicting). No flag = fallback to `config.harness`.
- `harness.rs`: command assembly. Claude: `claude --model <id>` + `ANTHROPIC_AUTH_TOKEN=ollama` (fixed literal), `ANTHROPIC_API_KEY=<api_key>`, `ANTHROPIC_BASE_URL=<claude_code.anthropic_base_url>`, `CLAUDE_CODE_MAX_CONTEXT_TOKENS=<context_length>` omitted when `None`. Opencode: `opencode --model <id>`, no envs.
- `app.rs`: `Model { id, name?, context_length? }`; picker shows `name` with `id` fallback and returns the full `Model`.
- `config.rs`: `Config::save` rewrites YAML and drops comments. Save happens only after model selection — cancellations never touch `config.yaml`.

## Gotchas

- `structure.md` is stale (lists `.env`, misses `cli.rs`/`harness.rs`/`config.rs`). Trust `src/` + `Cargo.toml`, not it.
- `.ai-jail` is a stray empty file at repo root; don't commit it.
- `roadmap.md` 2.0.0 lists `ANTHROPIC_DEFAULT_*` / `SUBAGENT_MODEL` envs — not implemented; the override fields exist in config but nothing exports them yet.
- Commits use Conventional Commits (`feat:`, `fix:`). Secrets rule: only `sk-test` (tests) and `sk-your-key-here` (example) may appear in diffs.
