pub mod curl;
pub mod docker;
pub mod ffmpeg;
pub mod find;
pub mod git;
pub mod tar;

/// Trait for smart command wrappers that support AI fallback semantics.
///
/// Implement this trait for each system command (curl, git, docker, etc.)
/// to plug into the unified `handle_wrapper` flow in main.
pub trait Wrapper: Sized + 'static {
    /// The system command name this wrapper targets (e.g. `"curl"`).
    fn name() -> &'static str;

    /// The argument to pass to the command to retrieve help text.
    fn help_arg() -> &'static str {
        "--help"
    }

    /// Optional AI prompt extension for wrapper-specific semantics.
    fn ai_prompt_extension() -> Option<String> {
        let help_text = std::process::Command::new(Self::name())
            .arg(Self::help_arg())
            .output()
            .ok()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_default();

        if help_text.trim().is_empty() {
            None
        } else {
            Some(format!(
                "This request targets the `{}` command.\n\nHere is its help output for reference:\n{}",
                Self::name(),
                help_text
            ))
        }
    }
}

/// Returns true if the command name has a built-in smart wrapper.
pub fn is_known_wrapper(command: &str) -> bool {
    command == curl::CurlWrapper::name()
        || command == git::GitWrapper::name()
        || command == docker::DockerWrapper::name()
        || command == tar::TarWrapper::name()
        || command == find::FindWrapper::name()
        || command == ffmpeg::FfmpegWrapper::name()
}
