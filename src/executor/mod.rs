use std::process::Stdio;
use tokio::process::Command;

/// A prepared command ready for execution.
pub struct PreparedCommand {
    pub program: String,
    pub args: Vec<String>,
    pub display: String,
}

/// Execute a prepared command, printing the resolved invocation and streaming output.
pub async fn execute(cmd: PreparedCommand) -> Result<(), String> {
    let mut child = Command::new(&cmd.program)
        .args(&cmd.args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("failed to spawn `{}`: {}", cmd.program, e))?;

    let status = child
        .wait()
        .await
        .map_err(|e| format!("failed to wait on `{}`: {}", cmd.program, e))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        return Err(format!("`{}` exited with code {}", cmd.program, code));
    }

    Ok(())
}
