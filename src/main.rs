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
    let cli_args = cli::Cli::parse();
    let config_path = cli_args
        .config
        .clone()
        .unwrap_or_else(config::Config::config_path);
    let mut config = config::Config::load(&config_path).map_err(|e| {
        let p = config_path.display();
        format!("{e} (copy config.example.yaml to {p} and fill it in)")
    })?;

    // Resolve harness early so we know which discovery to use.
    let harness = cli_args.harness_override().unwrap_or(config.harness);

    let models = match harness {
        config::Harness::ClaudeCode => {
            api::get_models(&config.api_key, &config.claude_code.anthropic_base_url)
                .await?
                .models()
        }
        config::Harness::Opencode => api::get_opencode_models()?,
        config::Harness::OmP => api::get_omp_models()?,
    };

    let app = app::App::new(models);
    let Some(model) = app.select_model()? else {
        println!("cancelled");
        return Ok(());
    };

    // Persist CLI override only after a model was selected, so
    // cancellations and pre-picker errors never rewrite config.
    if let Some(forced) = cli_args.harness_override()
        && forced != config.harness
    {
        config.harness = forced;
        config.save(&config_path)?;
    }

    // Validate required binaries are in PATH before attempting exec
    if let Err(e) = harness::validate_binaries(cli_args.no_jail, cli_args.no_memory) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    match harness {
        config::Harness::ClaudeCode => {
            let err = harness::build_claude_command(
                &model,
                &config,
                cli_args.no_jail,
                cli_args.no_memory,
            )
            .exec();
            Err(format!("failed to exec `claude`: {err}").into())
        }
        config::Harness::Opencode => {
            let err = harness::build_harness_command(
                "opencode",
                &model,
                cli_args.no_jail,
                cli_args.no_memory,
            )
            .exec();
            Err(format!("failed to exec `opencode`: {err}").into())
        }
        config::Harness::OmP => {
            let err =
                harness::build_harness_command("omp", &model, cli_args.no_jail, cli_args.no_memory)
                    .exec();
            Err(format!("failed to exec `omp`: {err}").into())
        }
    }
}
