use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub context_length: Option<u64>,
}

impl Model {
    /// Display name for the picker: `name` when present, `id` as fallback.
    pub fn display(&self) -> &str {
        self.name
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&self.id)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelResponse {
    data: Vec<Model>,
}

impl ModelResponse {
    pub fn models(self) -> Vec<Model> {
        self.data
    }
}

pub struct App {
    models: Vec<Model>,
}

impl App {
    pub fn new(models: Vec<Model>) -> Self {
        Self { models }
    }

    /// Show display names in fzf, return the selected model.
    pub fn select_model(&self) -> Result<Option<Model>, Box<dyn std::error::Error>> {
        let displays: Vec<String> = self
            .models
            .iter()
            .map(|m| m.display().to_string())
            .collect();
        let selected = crate::fzf::pick(displays)?;
        Ok(selected.and_then(|display| self.find_model(&display)))
    }

    fn find_model(&self, display: &str) -> Option<Model> {
        self.models.iter().find(|m| m.display() == display).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real API shape: extra fields ignored, `name` not always present.
    const RESPONSE: &str = r#"{
        "object": "list",
        "data": [
            {"id": "qwen/qwen3-next", "object": "model", "created": 1789312012,
             "owned_by": "x", "context_length": 1000, "name": "Qwen 3 Next"},
            {"id": "auto/best-coding", "object": "model", "created": 1789312012,
             "owned_by": "combo", "context_length": 1050000}
        ]
    }"#;

    #[test]
    fn decodes_response_with_missing_name() {
        let resp: ModelResponse = serde_json::from_str(RESPONSE).unwrap();
        let models = resp.models();
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].name.as_deref(), Some("Qwen 3 Next"));
        assert!(models[1].name.is_none());
    }

    #[test]
    fn display_falls_back_to_id() {
        let resp: ModelResponse = serde_json::from_str(RESPONSE).unwrap();
        let models = resp.models();
        assert_eq!(models[0].display(), "Qwen 3 Next");
        assert_eq!(models[1].display(), "auto/best-coding");
    }

    #[test]
    fn find_model_returns_match() {
        let resp: ModelResponse = serde_json::from_str(RESPONSE).unwrap();
        let app = App::new(resp.models());
        assert_eq!(app.find_model("Qwen 3 Next").unwrap().id, "qwen/qwen3-next");
        assert_eq!(
            app.find_model("auto/best-coding").unwrap().id,
            "auto/best-coding"
        );
        assert!(app.find_model("nope").is_none());
    }
}
