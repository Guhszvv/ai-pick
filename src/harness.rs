use std::process::Command;

use crate::app::Model;
use crate::config::Config;

/// Check if a binary exists in PATH.
fn check_binary(name: &str) -> Result<(), String> {
    which::which(name).map(|_| ()).map_err(|_| {
        format!(
            "`{}` not found in PATH. Install it or use --no-jail to skip wrappers.",
            name
        )
    })
}

/// Validate required binaries before exec.
pub fn validate_binaries(no_jail: bool, no_memory: bool) -> Result<(), String> {
    if no_jail {
        // When --no-jail is set, we exec the harness directly (no wrappers needed)
        return Ok(());
    }

    // ai-jail is always needed when not using --no-jail
    check_binary("ai-jail")?;

    // ai-memory is only needed when both wrappers are active
    if !no_memory {
        check_binary("ai-memory").map_err(|_| {
            format!(
                "`ai-memory` not found in PATH. Install it or use --no-memory to skip it:\n  ai-pick --no-memory"
            )
        })?;
    }

    Ok(())
}

/// Wrap a harness command with `ai-jail --gpu --network --agent-state [--env ...] ai-memory run`.
fn wrap_with_jail(harness: &str, harness_args: Vec<String>, envs: Vec<(String, String)>, no_memory: bool) -> Command {
    let mut cmd = Command::new("ai-jail");
    cmd.args(["--gpu", "--network", "--agent-state", "--no-status-bar"]);
    for (key, val) in &envs {
        cmd.args(["--env", &format!("{key}={val}")]);
    }
    if no_memory {
        cmd.arg(harness);
    } else {
        cmd.args(["ai-memory", "run", harness]);
    }
    cmd.args(harness_args);
    cmd
}

/// Resolve env vars for the claude harness.
fn claude_envs(model: &Model, config: &Config) -> Vec<(String, String)> {
    let api_key = config
        .claude_code
        .anthropic_api_key
        .clone()
        .unwrap_or_else(|| config.api_key.clone());
    let mut envs = vec![
        ("ANTHROPIC_AUTH_TOKEN".into(), "ollama".into()),
        ("ANTHROPIC_API_KEY".into(), api_key),
        ("ANTHROPIC_BASE_URL".into(), config.claude_code.anthropic_base_url.clone()),
    ];
    if let Some(context_length) = model.context_length {
        envs.push(("CLAUDE_CODE_MAX_CONTEXT_TOKENS".into(), context_length.to_string()));
    }
    for (env, val) in [
        ("ANTHROPIC_DEFAULT_OPUS_MODEL", &config.claude_code.anthropic_default_opus_model),
        ("ANTHROPIC_DEFAULT_SONNET_MODEL", &config.claude_code.anthropic_default_sonnet_model),
        ("ANTHROPIC_DEFAULT_HAIKU_MODEL", &config.claude_code.anthropic_default_haiku_model),
        ("CLAUDE_CODE_SUBAGENT_MODEL", &config.claude_code.claude_code_subagent_model),
    ] {
        if let Some(v) = val {
            envs.push((env.into(), v.clone()));
        }
    }
    envs
}

/// Build the `claude --model <id>` command, wrapped with ai-jail unless `no_jail`.
pub fn build_claude_command(model: &Model, config: &Config, no_jail: bool, no_memory: bool) -> Command {
    let envs = claude_envs(model, config);
    let args = vec!["--model".into(), model.id.clone()];
    if no_jail {
        let mut cmd = Command::new("claude");
        cmd.args(&args);
        for (key, val) in &envs {
            cmd.env(key, val);
        }
        cmd
    } else {
        wrap_with_jail("claude", args, envs, no_memory)
    }
}

/// Build a harness command (`<name> --model <id>`), wrapped with ai-jail unless `no_jail`.
pub fn build_harness_command(name: &str, model: &Model, no_jail: bool, no_memory: bool) -> Command {
    let args = vec!["--model".into(), model.id.clone()];
    if no_jail {
        let mut cmd = Command::new(name);
        cmd.args(&args);
        cmd
    } else {
        wrap_with_jail(name, args, vec![], no_memory)
    }
}

#[cfg(test)]
#[path = "tests/harness.rs"]
mod tests;
