use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct V0kConfig {
    pub api_base: String,
    pub api_key: Option<String>,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileConfig {
    pub api_base: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
}

impl V0kConfig {
    /// Load config with priority: env vars > ~/.v0k/config.toml > defaults.
    pub fn load() -> Self {
        let file_cfg = Self::load_file().unwrap_or_default();

        let api_base = Self::env_var("V0K_API_BASE")
            .or(file_cfg.api_base)
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        let api_key = Self::env_var("V0K_API_KEY").or(file_cfg.api_key);

        let model = Self::env_var("V0K_MODEL")
            .or(file_cfg.model)
            .unwrap_or_else(|| "gpt-4o-mini".to_string());

        Self {
            api_base,
            api_key,
            model,
        }
    }

    /// Returns true if an API key is configured (AI features available).
    pub fn has_ai(&self) -> bool {
        self.api_key.as_ref().is_some_and(|k| !k.trim().is_empty())
    }

    fn env_var(name: &str) -> Option<String> {
        std::env::var(name).ok()
    }

    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".v0k").join("config.toml"))
    }

    pub fn load_file() -> Option<FileConfig> {
        let path = Self::config_path()?;
        let content = std::fs::read_to_string(&path).ok()?;

        match toml::from_str(&content) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                eprintln!("warning: failed to parse {}: {e}", path.display());
                None
            }
        }
    }

    pub fn save_file(cfg: &FileConfig) -> std::io::Result<()> {
        let path = Self::config_path().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            )
        })?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(cfg)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

        std::fs::write(path, content)?;
        Ok(())
    }
}
