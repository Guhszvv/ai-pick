use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
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

    pub fn select_model(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let ids: Vec<String> = self.models.iter().map(|m| m.id.clone()).collect();
        Ok(crate::fzf::pick(ids)?)
    }
}
