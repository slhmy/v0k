use crate::wrappers::Wrapper;

/// Smart wrapper for `tar` — passes through standard tar invocations.
pub struct TarWrapper;

impl Wrapper for TarWrapper {
    fn name() -> &'static str {
        "tar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        if let Some(prompt) = TarWrapper::ai_prompt_extension() {
            assert!(prompt.contains("targets the `tar` command"));
        }
    }
}
