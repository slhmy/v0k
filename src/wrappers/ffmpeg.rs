use crate::wrappers::Wrapper;

/// Smart wrapper for `ffmpeg` — passes through standard ffmpeg invocations.
pub struct FfmpegWrapper;

impl Wrapper for FfmpegWrapper {
    fn name() -> &'static str {
        "ffmpeg"
    }

    fn help_arg() -> &'static str {
        "-help"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        if let Some(prompt) = FfmpegWrapper::ai_prompt_extension() {
            assert!(prompt.contains("targets the `ffmpeg` command"));
        }
    }
}
