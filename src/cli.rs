use clap::Parser;
use std::path::PathBuf;

use crate::config::Harness;

/// Pick a model, then exec the harness (`claude-code` default from config).
#[derive(Debug, Parser)]
#[command(name = "ai-pick", about = "Pick a model and exec the harness")]
pub struct Cli {
    /// Use the claude-code harness for this run (also persisted).
    #[arg(long, conflicts_with_all = ["opencode", "omp"])]
    pub claude_code: bool,
    /// Use the opencode harness for this run (also persisted).
    #[arg(long, conflicts_with_all = ["claude_code", "omp"])]
    pub opencode: bool,
    /// Use the oh-my-pi (omp) harness for this run (also persisted).
    #[arg(long, conflicts_with_all = ["claude_code", "opencode"])]
    pub omp: bool,
    /// Skip ai-jail wrapper — exec the harness directly.
    #[arg(long)]
    pub no_jail: bool,
    /// Skip ai-memory wrapper — wrap with ai-jail but exec the harness directly.
    #[arg(long)]
    pub no_memory: bool,
    /// Path to config file (default: ~/.config/ai-pick/config.yaml).
    #[arg(long)]
    pub config: Option<PathBuf>,
}

impl Cli {
    /// Harness forced via CLI flag, if any.
    pub fn harness_override(&self) -> Option<Harness> {
        if self.claude_code {
            Some(Harness::ClaudeCode)
        } else if self.opencode {
            Some(Harness::Opencode)
        } else if self.omp {
            Some(Harness::OmP)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_flag_means_no_override() {
        let cli = Cli::try_parse_from(["ai-pick"]).unwrap();
        assert!(cli.harness_override().is_none());
    }

    #[test]
    fn claude_code_flag_overrides() {
        let cli = Cli::try_parse_from(["ai-pick", "--claude-code"]).unwrap();
        assert_eq!(cli.harness_override(), Some(Harness::ClaudeCode));
    }

    #[test]
    fn opencode_flag_overrides() {
        let cli = Cli::try_parse_from(["ai-pick", "--opencode"]).unwrap();
        assert_eq!(cli.harness_override(), Some(Harness::Opencode));
    }

    #[test]
    fn omp_flag_overrides() {
        let cli = Cli::try_parse_from(["ai-pick", "--omp"]).unwrap();
        assert_eq!(cli.harness_override(), Some(Harness::OmP));
    }

    #[test]
    fn conflicting_flags_are_rejected() {
        assert!(Cli::try_parse_from(["ai-pick", "--claude-code", "--opencode"]).is_err());
        assert!(Cli::try_parse_from(["ai-pick", "--claude-code", "--omp"]).is_err());
        assert!(Cli::try_parse_from(["ai-pick", "--opencode", "--omp"]).is_err());
    }
}
