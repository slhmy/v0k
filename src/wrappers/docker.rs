use crate::wrappers::Wrapper;

/// Smart wrapper for `docker` — passes through standard docker invocations.
pub struct DockerWrapper;

impl Wrapper for DockerWrapper {
    fn name() -> &'static str {
        "docker"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_extension_contains_help() {
        if let Some(prompt) = DockerWrapper::ai_prompt_extension() {
            assert!(prompt.contains("targets the `docker` command"));
        }
    }
}
