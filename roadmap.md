# 1.0.0
- [x] Model picker
  - [x] Utilizar o model_name e model_id como fallback
  - [x] Estilizar o fzf
- [x] Montagem do comando com ai-jail + ai-memory

# 2.0.0
- [x] Opção de configurar environments do claude *(evitar o claude chamando subagents com modelos proprietários)*:

```
ANTHROPIC_BASE_URL              # endpoint alternativo
ANTHROPIC_AUTH_TOKEN            # token do endpoint alternativo
ANTHROPIC_DEFAULT_OPUS_MODEL    # modelo a invocar no lugar de Opus
ANTHROPIC_DEFAULT_SONNET_MODEL  # idem pra Sonnet
ANTHROPIC_DEFAULT_HAIKU_MODEL   # idem pra Haiku
CLAUDE_CODE_SUBAGENT_MODEL      # modelo do subagent
```

Inspiração: [DeepClaude - AkitaOnRails](https://akitaonrails.com/2026/05/04/llm-benchmarks-deepseek-unlocked-deepclaude/#o-que-%c3%a9-deepclaude)

