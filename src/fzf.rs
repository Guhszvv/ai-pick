use std::io::Write;
use std::process::{Command, Stdio};

/// Pipe `options` (one per line) into external `fzf` and return selected line.
/// Returns `Ok(None)` when user cancels (Esc / Ctrl-C, fzf exit 130/1 with no output).
pub fn pick(options: Vec<String>) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if options.is_empty() {
        return Ok(None);
    }

    let mut child = Command::new("fzf")
        .arg("--prompt=Search: ")
        .arg("--layout=reverse")
        .arg("--header=↑↓ navegar   Enter selecionar   Esc sair")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            format!("failed to spawn `fzf` (is it installed?): {e}")
        })?;

    if let Some(stdin) = child.stdin.as_mut() {
        for opt in &options {
            writeln!(stdin, "{opt}")?;
        }
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected.is_empty() {
        return Ok(None);
    }
    Ok(Some(selected))
}
