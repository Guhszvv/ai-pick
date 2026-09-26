use clap::error::Result;
use std::process::Command;

use crate::app::{Model, ModelResponse};

pub async fn get_models(api_key: &str, base_url: &str) -> Result<ModelResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{base_url}/v1/models");
    let response = client.get(&url).bearer_auth(api_key).send().await?;

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

/// Shape of `omp models --json`: `{"models":[{"id","name","contextWindow",...}]}`.
#[derive(serde::Deserialize)]
struct OmpModel {
    id: String,
    name: Option<String>,
    #[serde(rename = "contextWindow")]
    context_length: Option<u64>,
}

#[derive(serde::Deserialize)]
struct OmpList {
    models: Vec<OmpModel>,
}

impl From<OmpModel> for Model {
    fn from(m: OmpModel) -> Self {
        Model {
            id: m.id,
            name: m.name,
            context_length: m.context_length,
        }
    }
}

pub fn get_omp_models() -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let output = Command::new("omp")
        .args(["models", "--json"])
        .output()
        .map_err(|e| format!("failed to run `omp models --json`: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("`omp models --json` failed: {stderr}").into());
    }

    let list: OmpList = serde_json::from_slice(&output.stdout)?;

    Ok(list.models.into_iter().map(Model::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OMP_RESPONSE: &str = r#"{
        "models": [
            {"id": "aug/fable-5", "name": "aug/Claude Fable 5", "contextWindow": 200000,
             "provider": "omni", "kind": "chat", "cost": {"input": 0, "output": 0}},
            {"id": "no-name-model", "name": null, "contextWindow": null}
        ]
    }"#;

    #[test]
    fn decodes_omp_models() {
        let list: OmpList = serde_json::from_str(OMP_RESPONSE).unwrap();
        let models: Vec<Model> = list.models.into_iter().map(Model::from).collect();
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id, "aug/fable-5");
        assert_eq!(models[0].display(), "aug/Claude Fable 5");
        assert_eq!(models[0].context_length, Some(200000));
        assert!(models[1].name.is_none());
        assert!(models[1].context_length.is_none());
        assert_eq!(models[1].display(), "no-name-model");
    }
}
