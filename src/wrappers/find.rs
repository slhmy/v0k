use crate::wrappers::Wrapper;

/// Smart wrapper for `find` — passes through standard find invocations.
pub struct FindWrapper;

impl Wrapper for FindWrapper {
    fn name() -> &'static str {
        "find"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        if let Some(prompt) = FindWrapper::ai_prompt_extension() {
            assert!(prompt.contains("targets the `find` command"));
        }
    }
}
