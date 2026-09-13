mod api;
mod app;
mod cli;
mod config;
mod fzf;
mod harness;

use clap::Parser;
use std::os::unix::process::CommandExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CONFIG_PATH: &str = "config.yaml";
    let cli_args = cli::Cli::parse();
    let mut config = config::Config::load(CONFIG_PATH).map_err(|e| {
        format!("{e} (copy {CONFIG_PATH} from config.example.yaml and fill it in)")
    })?;

    let models = api::get_models(&config.api_key).await?.models();

    let app = app::App::new(models);
    let Some(model) = app.select_model()? else {
        println!("cancelled");
        return Ok(());
    };

    // Persist CLI override only after a model was selected, so
    // cancellations and pre-picker errors never rewrite config.yaml.
    let mut harness = config.harness;
    if let Some(forced) = cli_args.harness_override() {
        harness = forced;
        if forced != config.harness {
            config.harness = forced;
            config.save(CONFIG_PATH)?;
        }
    }

    match harness {
        config::Harness::ClaudeCode => {
            let err = harness::build_claude_command(&model, &config).exec();
            Err(format!("failed to exec `claude`: {err}").into())
        }
        config::Harness::Opencode => {
            let err = harness::build_opencode_command(&model).exec();
            Err(format!("failed to exec `opencode`: {err}").into())
        }
    }
}
