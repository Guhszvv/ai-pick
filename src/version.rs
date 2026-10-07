//! Version checker module for ai-pick.
//!
//! Usage: `ai-pick --version-debug 2.0.0`
//!
//! Compares the local version with the provided debug version (or latest from GitHub),
//! and displays an update message if a newer version is available.

use semver::Version;

/// Represents the result of comparing two versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    /// Local version is newer or equal
    SameOrNewer,
    /// Remote/debug version is newer
    NeedsUpdate,
}

impl UpdateStatus {
    pub fn is_newer_needed(&self) -> bool {
        matches!(self, UpdateStatus::NeedsUpdate)
    }
}

/// Returns the local version from Cargo.toml.
///
/// Uses `env!("CARGO_PKG_VERSION")` which is set at compile time.
pub fn get_local_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Fetches the latest version from GitHub releases API.
pub async fn get_latest_version() -> Result<Option<String>, reqwest::Error> {
    let url = "https://api.github.com/repos/Guhszvv/ai-pick/releases/latest";
    // Short timeout so a slow GitHub never delays launch.
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_default();
    let response = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            concat!("ai-pick/", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await?;
    if !response.status().is_success() {
        return Ok(None);
    }
    let json: serde_json::Value = response.json().await?;
    let tag = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    Ok(tag)
}

/// Compares two versions and returns the update status.
fn compare_versions(local: &str, remote: &str) -> UpdateStatus {
    // Tolerate a leading `v` in GitHub release tags (e.g. `v2.1.0`).
    let local_ver: Version = local.trim_start_matches('v').parse().unwrap_or_else(|_| Version::parse("0.0.0").unwrap());
    let remote_ver: Version = remote.trim_start_matches('v').parse().unwrap_or_else(|_| Version::parse("0.0.0").unwrap());

    match local_ver.cmp(&remote_ver) {
        std::cmp::Ordering::Greater => UpdateStatus::SameOrNewer,
        std::cmp::Ordering::Equal => UpdateStatus::SameOrNewer,
        std::cmp::Ordering::Less => UpdateStatus::NeedsUpdate,
    }
}

/// Shows an update message if a newer version is available.
pub fn show_update_message(local: &str, remote: &str) -> Option<String> {
    let status = compare_versions(local, remote);

    if status.is_newer_needed() {
        let setup_cmd = "curl -fsSL https://raw.githubusercontent.com/Guhszvv/ai-pick/master/install.sh | bash";
        let msg = format!(
            "New version {} available! Run: {}\n",
            remote, setup_cmd
        );
        println!("{}", msg);
        return Some(msg);
    }
    None
}

/// Main entry point for version checking with debug support.
///
/// If `debug_version` is provided, compares local vs debug version.
/// Otherwise, fetches latest from GitHub.
pub async fn check_version(debug_version: Option<&str>) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let local = get_local_version();
    let remote = match debug_version {
        Some(debug_ver) => debug_ver.to_string(),
        // Network failures must never block the program: fall back to local.
        None => get_latest_version()
            .await
            .unwrap_or_default()
            .unwrap_or_else(|| local.to_string()),
    };

    // Debug mode always reports the comparison so it can be validated.
    if debug_version.is_some() {
        let newer = compare_versions(local, &remote).is_newer_needed();
        println!(
            "version-debug: local {local}, compared {remote} -> {}",
            if newer { "update available" } else { "up to date" }
        );
    }

    Ok(show_update_message(local, &remote))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn older_remote_needs_update() {
        assert_eq!(compare_versions("2.0.0", "3.0.0"), UpdateStatus::NeedsUpdate);
    }

    #[test]
    fn equal_or_newer_local_is_fine() {
        assert_eq!(compare_versions("2.0.0", "2.0.0"), UpdateStatus::SameOrNewer);
        assert_eq!(compare_versions("2.1.0", "2.0.0"), UpdateStatus::SameOrNewer);
    }

    #[test]
    fn leading_v_tag_is_tolerated() {
        assert_eq!(compare_versions("2.0.0", "v3.0.0"), UpdateStatus::NeedsUpdate);
    }

    #[test]
    fn unparseable_remote_never_blocks() {
        assert_eq!(compare_versions("2.0.0", "garbage"), UpdateStatus::SameOrNewer);
        assert!(show_update_message("2.0.0", "garbage").is_none());
    }

    #[test]
    fn update_message_mentions_setup_command() {
        let msg = show_update_message("2.0.0", "3.0.0").expect("should warn");
        assert!(msg.contains("install.sh"));
    }
}
