use crate::app::ModelResponse;

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
