# How to add new harness

## Questions
1. How to invoke the harness in terminal? Example: `claude`, `opencode`, `pi`
2. How to pass model for harness? Example: `claude --model qwen3.6-14b-a3b`
3. harness have self model discover/list method? Example: `opencode models`

## How to implement
1. Add cli argument in `cli.rs` in Struct and impl
2. Add harness in `config.rs`, Harness Enum and Display
3. Add harness in `main.rs` on model get method and harness match
