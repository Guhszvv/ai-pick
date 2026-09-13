use std::process::Command;

use crate::app::Model;
use crate::config::Config;

/// Build the `claude --model <id>` command with the mandatory env vars.
///
/// - `ANTHROPIC_AUTH_TOKEN=ollama` (fixed)
/// - `ANTHROPIC_API_KEY` from `config.api_key`
/// - `ANTHROPIC_BASE_URL` from `config.claude_code.anthropic_base_url`
/// - `CLAUDE_CODE_MAX_CONTEXT_TOKENS` from the picker's `context_length`
///   (omitted when the model has none)
pub fn build_claude_command(model: &Model, config: &Config) -> Command {
    let mut cmd = Command::new("claude");
    cmd.arg("--model").arg(&model.id);
    cmd.env("ANTHROPIC_AUTH_TOKEN", "ollama");
    cmd.env("ANTHROPIC_API_KEY", &config.api_key);
    cmd.env(
        "ANTHROPIC_BASE_URL",
        &config.claude_code.anthropic_base_url,
    );
    if let Some(context_length) = model.context_length {
        cmd.env(
            "CLAUDE_CODE_MAX_CONTEXT_TOKENS",
            context_length.to_string(),
        );
    }
    cmd
}

/// Build the `opencode --model <id>` command.
///
/// Unlike claude-code, opencode needs none of the `ANTHROPIC_*` envs:
/// just the `--model` flag.
pub fn build_opencode_command(model: &Model) -> Command {
    let mut cmd = Command::new("opencode");
    cmd.arg("--model").arg(&model.id);
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ClaudeCodeConfig, Harness};

    fn fixture_config() -> Config {
        Config {
            api_key: "sk-test".to_string(),
            harness: Harness::ClaudeCode,
            claude_code: ClaudeCodeConfig {
                anthropic_base_url: "http://localhost:20128".to_string(),
                anthropic_default_opus_model: None,
                anthropic_default_sonnet_model: None,
                anthropic_default_haiku_model: None,
                claude_code_subagent_model: None,
            },
        }
    }

    fn env_of(cmd: &Command) -> std::collections::HashMap<String, String> {
        cmd.get_envs()
            .filter_map(|(k, v)| {
                Some((
                    k.to_str()?.to_string(),
                    v?.to_str()?.to_string(),
                ))
            })
            .collect()
    }

    #[test]
    fn exports_mandatory_envs_and_model_flag() {
        let config = fixture_config();
        let model = Model {
            id: "qwen/qwen3-next".to_string(),
            name: Some("Qwen 3 Next".to_string()),
            context_length: Some(1000),
        };
        let cmd = build_claude_command(&model, &config);

        let args: Vec<_> = cmd
            .get_args()
            .map(|a| a.to_str().unwrap().to_string())
            .collect();
        assert_eq!(args, vec!["--model", "qwen/qwen3-next"]);

        let env = env_of(&cmd);
        assert_eq!(env.get("ANTHROPIC_AUTH_TOKEN").map(String::as_str), Some("ollama"));
        assert_eq!(env.get("ANTHROPIC_API_KEY").map(String::as_str), Some("sk-test"));
        assert_eq!(
            env.get("ANTHROPIC_BASE_URL").map(String::as_str),
            Some("http://localhost:20128")
        );
        assert_eq!(
            env.get("CLAUDE_CODE_MAX_CONTEXT_TOKENS").map(String::as_str),
            Some("1000")
        );
    }

    #[test]
    fn omits_max_context_tokens_when_missing() {
        let config = fixture_config();
        let model = Model {
            id: "auto/best-coding".to_string(),
            name: None,
            context_length: None,
        };
        let cmd = build_claude_command(&model, &config);
        let env = env_of(&cmd);
        assert!(!env.contains_key("CLAUDE_CODE_MAX_CONTEXT_TOKENS"));
        assert_eq!(env.get("ANTHROPIC_API_KEY").map(String::as_str), Some("sk-test"));
    }

    #[test]
    fn opencode_passes_model_without_anthropic_envs() {
        let model = Model {
            id: "qwen/qwen3-next".to_string(),
            name: Some("Qwen 3 Next".to_string()),
            context_length: Some(1000),
        };
        let cmd = build_opencode_command(&model);

        let args: Vec<_> = cmd
            .get_args()
            .map(|a| a.to_str().unwrap().to_string())
            .collect();
        assert_eq!(args, vec!["--model", "qwen/qwen3-next"]);

        let env = env_of(&cmd);
        assert!(env.keys().all(|k| !k.starts_with("ANTHROPIC_")));
        assert!(!env.contains_key("CLAUDE_CODE_MAX_CONTEXT_TOKENS"));
    }
}
