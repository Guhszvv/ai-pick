use std::process::Command;

use crate::app::{Model, ModelResponse};

pub async fn get_models(
    api_key: &str,
    base_url: &str,
) -> Result<ModelResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{base_url}/v1/models");
    let response = client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await?;

    Ok(response.json().await?)
}

/// Discover opencode models via `opencode models` (one ID per line).
pub fn get_opencode_models() -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let output = Command::new("opencode")
        .args(["models"])
        .output()
        .map_err(|e| format!("failed to run `opencode models`: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("`opencode models` failed: {stderr}").into());
    }

    let models = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|id| Model {
            id,
            name: None,
            context_length: None,
        })
        .collect();

    Ok(models)
}

