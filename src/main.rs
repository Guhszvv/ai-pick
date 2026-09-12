use std::env;

mod api;
mod app;
mod fzf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let models = api::get_models(api_key).await?.models();

    let app = app::App::new(models);
    match app.select_model()? {
        Some(id) => println!("{id}"),
        None => println!("cancelled"),
    }

    Ok(())
}
