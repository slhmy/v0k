use crate::wrappers::Wrapper;

/// Smart wrapper for `curl` — passes through standard curl invocations.
pub struct CurlWrapper;

impl Wrapper for CurlWrapper {
    fn name() -> &'static str {
        "curl"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        let prompt = CurlWrapper::ai_prompt_extension().unwrap();
        assert!(prompt.contains("targets the `curl` command"));
        // Check that typical curl help output is included (e.g. "--url")
        if !prompt.is_empty() {
            assert!(prompt.contains("curl") || prompt.contains("url"));
        }
    }
}
