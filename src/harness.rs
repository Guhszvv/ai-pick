use std::process::Command;

use crate::app::Model;
use crate::config::Config;

/// Wrap a harness command with `ai-jail --gpu --network --agent-state [--env ...] ai-memory run`.
fn wrap_with_jail(harness: &str, harness_args: Vec<String>, envs: Vec<(String, String)>) -> Command {
    let mut cmd = Command::new("ai-jail");
    cmd.args(["--gpu", "--network", "--agent-state", "--no-status-bar"]);
    for (key, val) in &envs {
        cmd.args(["--env", &format!("{key}={val}")]);
    }
    cmd.args(["ai-memory", "run", harness]);
    cmd.args(harness_args);
    cmd
}

/// Build the `ai-jail ... ai-memory run claude --model <id>` command with the mandatory env vars.
pub fn build_claude_command(model: &Model, config: &Config) -> Command {
    let mut envs = vec![
        ("ANTHROPIC_AUTH_TOKEN".into(), "ollama".into()),
        ("ANTHROPIC_API_KEY".into(), config.api_key.clone()),
        ("ANTHROPIC_BASE_URL".into(), config.claude_code.anthropic_base_url.clone()),
    ];
    if let Some(context_length) = model.context_length {
        envs.push(("CLAUDE_CODE_MAX_CONTEXT_TOKENS".into(), context_length.to_string()));
    }
    wrap_with_jail("claude", vec!["--model".into(), model.id.clone()], envs)
}

/// Build the `ai-jail ... ai-memory run opencode --model <id>` command.
pub fn build_opencode_command(model: &Model) -> Command {
    wrap_with_jail("opencode", vec!["--model".into(), model.id.clone()], vec![])
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

    #[test]
    fn claude_wrapped_with_jail_and_envs() {
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
        assert_eq!(
            args,
            vec![
                "--gpu", "--network", "--agent-state", "--no-status-bar",
                "--env", "ANTHROPIC_AUTH_TOKEN=ollama",
                "--env", "ANTHROPIC_API_KEY=sk-test",
                "--env", "ANTHROPIC_BASE_URL=http://localhost:20128",
                "--env", "CLAUDE_CODE_MAX_CONTEXT_TOKENS=1000",
                "ai-memory", "run",
                "claude", "--model", "qwen/qwen3-next"
            ]
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

        let args: Vec<_> = cmd
            .get_args()
            .map(|a| a.to_str().unwrap().to_string())
            .collect();
        assert!(!args.iter().any(|a| a.contains("CLAUDE_CODE_MAX_CONTEXT_TOKENS")));
        assert!(args.contains(&"--env".to_string()));
        assert!(args.iter().any(|a| a == "ANTHROPIC_API_KEY=sk-test"));
    }

    #[test]
    fn opencode_wrapped_with_jail_without_anthropic_envs() {
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
        assert_eq!(
            args,
            vec![
                "--gpu", "--network", "--agent-state", "--no-status-bar",
                "ai-memory", "run",
                "opencode", "--model", "qwen/qwen3-next"
            ]
        );
        assert!(!args.iter().any(|a| a.starts_with("ANTHROPIC_")));
    }
}
