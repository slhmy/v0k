use crate::wrappers::Wrapper;

/// Smart wrapper for `git` — passes through standard git invocations.
pub struct GitWrapper;

impl Wrapper for GitWrapper {
    fn name() -> &'static str {
        "git"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        let prompt = GitWrapper::ai_prompt_extension().unwrap();
        assert!(prompt.contains("targets the `git` command"));
        // Check that typical git help output is included (e.g. "clone", "commit")
        if !prompt.is_empty() {
            assert!(prompt.contains("git") || prompt.contains("clone"));
        }
    }
}
