# ai-pick

CLI to pick an LLM model via `fzf` and launch a harness (`claude-code`, `opencode`, or `omp`) with the selected model.

## How it works

```
ai-pick → fetch models → fzf picker → exec harness with --model
```

`ai-pick` opens an interactive selector with `fzf` and replaces the process with the harness using the selected model (via `exec` — does not return on success).

- **Claude Code** and **oh-my-pi**: models are fetched from a custom endpoint.
- **Opencode**: models are discovered via `opencode models`.

The command is wrapped with `ai-jail` + `ai-memory` for sandboxing and long-term memory. Use `--no-jail` to skip the wrapper.

## Setup

```bash
./install.sh
```

This builds the release binary, copies it to `~/.local/bin/`, and creates `~/.config/ai-pick/config.yaml` from the example (only if it doesn't exist yet). Run again anytime to update.

Make sure `~/.local/bin` is in your `$PATH`.

External dependencies:
- `fzf` — binary spawned by `ai-pick`
- `claude` or `opencode` — the harness to execute
- `ai-jail` / `ai-memory` — sandbox and memory wrappers
- Custom models endpoint (only for Claude Code)

## Configuration

`~/.config/ai-pick/config.yaml`:

```yaml
api_key: "sk-your-key"
harness: "claude-code"  # claude-code | opencode | omp (last harness used)

claude_code:
  anthropic_base_url: "http://localhost:20128"
  # Optional model overrides for subagents:
  # anthropic_default_opus_model: "..."
  # anthropic_default_sonnet_model: "..."
  # anthropic_default_haiku_model: "..."
  # claude_code_subagent_model: "..."
```

## Usage

```bash
# default — uses the harness configured in config.yaml
ai-pick

# force claude-code for this run
ai-pick --claude-code

# force opencode for this run
ai-pick --opencode

# force oh-my-pi for this run
ai-pick --omp

# skip ai-jail wrapper — exec harness directly
ai-pick --no-jail

# skip ai-memory wrapper - exec ai-jail + harness
ai-pick --no-memory

# use a custom config file
ai-pick --config ./my-config.yaml
```

The harness choice via flag is persisted in `config.yaml` (only after model selection — cancellations never touch the config).

## License

[MIT](LICENSE)
