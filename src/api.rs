use crate::app::ModelResponse;

pub async fn get_models(api_key: String) -> Result<ModelResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:20128/v1/models")
        .bearer_auth(api_key)
        .send()
        .await?;

    Ok(response.json().await?)
}
