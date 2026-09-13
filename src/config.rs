use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

/// Last harness used by the app. Decided on first run (wizard — follow-up),
/// persisted afterwards via [`Config::save`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Harness {
    ClaudeCode,
    Opencode,
}

impl fmt::Display for Harness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Harness::ClaudeCode => "claude-code",
            Harness::Opencode => "opencode",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaudeCodeConfig {
    pub anthropic_base_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anthropic_default_opus_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anthropic_default_sonnet_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anthropic_default_haiku_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claude_code_subagent_model: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    pub harness: Harness,
    pub claude_code: ClaudeCodeConfig,
}

impl Config {
    /// Load config from a YAML file (e.g. `config.yaml`).
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let mut cfg: Self = serde_yaml::from_str(&raw)
            .map_err(|e| format!("invalid YAML in {}: {e}", path.display()))?;
        cfg.normalize();
        Ok(cfg)
    }

    /// Persist config back to disk (used to store last-used harness).
    /// Note: rewrites the file, YAML comments are not preserved.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let raw = serde_yaml::to_string(self)?;
        std::fs::write(path.as_ref(), raw)?;
        Ok(())
    }

    /// `ANTHROPIC_AUTH_TOKEN` reuses the top-level `api_key` — no duplication.
    pub fn auth_token(&self) -> &str {
        &self.api_key
    }

    /// Empty-string model overrides behave like absent ones (env var not exported).
    fn normalize(&mut self) {        let c = &mut self.claude_code;
        for field in [
            &mut c.anthropic_default_opus_model,
            &mut c.anthropic_default_sonnet_model,
            &mut c.anthropic_default_haiku_model,
            &mut c.claude_code_subagent_model,
        ] {
            if field.as_deref().is_some_and(str::is_empty) {
                *field = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
api_key: "sk-test"
harness: "claude-code"
claude_code:
  anthropic_base_url: "http://localhost:20128"
  anthropic_default_opus_model: "my-opus"
"#;

    #[test]
    fn parses_valid_config() {
        let cfg: Config = serde_yaml::from_str(VALID).unwrap();
        assert_eq!(cfg.api_key, "sk-test");
        assert_eq!(cfg.harness, Harness::ClaudeCode);
        assert_eq!(cfg.auth_token(), "sk-test");
        assert_eq!(
            cfg.claude_code.anthropic_default_opus_model.as_deref(),
            Some("my-opus")
        );
        assert!(cfg.claude_code.anthropic_default_sonnet_model.is_none());
    }

    #[test]
    fn rejects_unknown_harness() {
        let raw = VALID.replace("claude-code", "unknown-harness");
        assert!(serde_yaml::from_str::<Config>(&raw).is_err());
    }

    #[test]
    fn empty_model_string_normalizes_to_none() {
        let raw = r#"
api_key: "sk-test"
harness: "opencode"
claude_code:
  anthropic_base_url: "http://localhost:20128"
  anthropic_default_haiku_model: ""
"#;
        let mut cfg: Config = serde_yaml::from_str(raw).unwrap();
        cfg.normalize();
        assert_eq!(cfg.harness, Harness::Opencode);
        assert!(cfg.claude_code.anthropic_default_haiku_model.is_none());
    }

    #[test]
    fn missing_file_errors() {
        assert!(Config::load("nonexistent-config.yaml").is_err());
    }
}
