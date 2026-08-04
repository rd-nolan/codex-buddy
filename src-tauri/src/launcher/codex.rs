use std::process::Command;

/// Launch Codex with CDP enabled.
pub fn launch_codex(port: u16) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args([
                "/Applications/Codex.app",
                "--args",
                &format!("--remote-debugging-port={}", port),
            ])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("Codex.exe")
            .arg(format!("--remote-debugging-port={}", port))
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
