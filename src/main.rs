mod api;
mod app;
mod config;
mod fzf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CONFIG_PATH: &str = "config.yaml";
    let config = config::Config::load(CONFIG_PATH).map_err(|e| {
        format!("{e} (copy {CONFIG_PATH} from config.example.yaml and fill it in)")
    });

    let config = config?;

    let models = api::get_models(&config.api_key).await?.models();

    let app = app::App::new(models);
    match app.select_model()? {
        Some(id) => println!("{id}"),
        None => println!("cancelled"),
    }

    Ok(())
}
