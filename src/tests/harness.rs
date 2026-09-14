use super::*;
use crate::config::{ClaudeCodeConfig, Harness};

fn fixture_config() -> Config {
    Config {
        api_key: "sk-test".to_string(),
        harness: Harness::ClaudeCode,
        claude_code: ClaudeCodeConfig {
            anthropic_api_key: None,
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
    let cmd = build_claude_command(&model, &config, false);

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
    let cmd = build_claude_command(&model, &config, false);

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
    let cmd = build_opencode_command(&model, false);

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

#[test]
fn exports_optional_model_overrides() {
    let mut config = fixture_config();
    config.claude_code.anthropic_default_opus_model = Some("local-opus".into());
    config.claude_code.anthropic_default_sonnet_model = Some("local-sonnet".into());
    config.claude_code.anthropic_default_haiku_model = Some("local-haiku".into());
    config.claude_code.claude_code_subagent_model = Some("local-sub".into());
    let model = Model {
        id: "qwen/qwen3-next".to_string(),
        name: None,
        context_length: None,
    };
    let cmd = build_claude_command(&model, &config, false);

    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();
    assert!(args.iter().any(|a| a == "ANTHROPIC_DEFAULT_OPUS_MODEL=local-opus"));
    assert!(args.iter().any(|a| a == "ANTHROPIC_DEFAULT_SONNET_MODEL=local-sonnet"));
    assert!(args.iter().any(|a| a == "ANTHROPIC_DEFAULT_HAIKU_MODEL=local-haiku"));
    assert!(args.iter().any(|a| a == "CLAUDE_CODE_SUBAGENT_MODEL=local-sub"));
}

#[test]
fn anthropic_api_key_override() {
    let mut config = fixture_config();
    config.claude_code.anthropic_api_key = Some("sk-custom-key".into());
    let model = Model {
        id: "qwen/qwen3-next".to_string(),
        name: None,
        context_length: None,
    };
    let cmd = build_claude_command(&model, &config, false);

    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();
    assert!(args.iter().any(|a| a == "ANTHROPIC_API_KEY=sk-custom-key"));
}

#[test]
fn claude_no_jail_skips_wrapper() {
    let config = fixture_config();
    let model = Model {
        id: "qwen/qwen3-next".to_string(),
        name: None,
        context_length: None,
    };
    let cmd = build_claude_command(&model, &config, true);

    let prog = cmd.get_program().to_str().unwrap();
    assert_eq!(prog, "claude");

    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();
    assert_eq!(args, vec!["--model", "qwen/qwen3-next"]);
}

#[test]
fn opencode_no_jail_skips_wrapper() {
    let model = Model {
        id: "qwen/qwen3-next".to_string(),
        name: None,
        context_length: None,
    };
    let cmd = build_opencode_command(&model, true);

    let prog = cmd.get_program().to_str().unwrap();
    assert_eq!(prog, "opencode");

    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();
    assert_eq!(args, vec!["--model", "qwen/qwen3-next"]);
}
