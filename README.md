# ai-pick

CLI to pick an LLM model via `fzf` and launch a harness (`claude-code` or `opencode`) with the selected model.

## How it works

```
ai-pick → fetch models → fzf picker → exec harness with --model
```

`ai-pick` fetches models from a local endpoint (`localhost:20128/v1/models`), opens an interactive selector with `fzf`, and replaces the process with the harness using the selected model (via `exec` — does not return on success).

The command is wrapped with `ai-jail` + `ai-memory` for sandboxing and long-term memory.

## Setup

```bash
cp config.example.yaml config.yaml
# edit config.yaml — fill in api_key
```

External dependencies:
- `fzf` — binary spawned by `ai-pick`
- `claude` or `opencode` — the harness to execute
- `ai-jail` / `ai-memory` — sandbox and memory wrappers
- Models endpoint running at `http://localhost:20128`

## Configuration

`config.yaml`:

```yaml
api_key: "sk-your-key"
harness: "claude-code"  # claude-code | opencode

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
```

The harness choice via flag is persisted in `config.yaml` (only after model selection — cancellations never touch the config).

## License

MIT
