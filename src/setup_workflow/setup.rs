use std::path::Path;

use crate::{
    api,
    app::App,
    config::{
        Config,
        Harness::{self, ClaudeCode},
    },
    harness::validate_harness,
};

pub async fn setup_claude_code(
    arg: Harness,
    config: &mut Config,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // support only for claude-code yet
    if !validate_harness(arg, Some(ClaudeCode)) {
        return Err(format!("harness `{arg}` not supported (only claude-code)").into());
    }

    let models = api::get_models(&config.api_key, &config.claude_code.anthropic_base_url)
        .await?
        .models();
    let app = App::new(models);

    let c = &mut config.claude_code;
    for (label, field) in [
        ("Select the Opus model", &mut c.anthropic_default_opus_model),
        (
            "Select the Sonnet model",
            &mut c.anthropic_default_sonnet_model,
        ),
        (
            "Select the Haiku model",
            &mut c.anthropic_default_haiku_model,
        ),
        (
            "Select the Subagent model",
            &mut c.claude_code_subagent_model,
        ),
    ] {
        if let Some(model) = app.select_with_header(label)? {
            *field = Some(model.id);
        }
    }

    config.save(path)?;
    Ok(())
}
