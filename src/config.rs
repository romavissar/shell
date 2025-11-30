use serde::Deserialize;
use std::fs;

fn default_true() -> bool { true }

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub show_user: bool,
    #[serde(default = "default_true")]
    pub show_directory: bool,
    #[serde(default = "default_true")]
    pub show_git: bool,
    #[serde(default)]
    pub prompt_char: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_user: false,
            show_directory: true,
            show_git: true,
            prompt_char: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        dirs::home_dir()
            .map(|h| h.join(".shell.toml"))
            .and_then(|p| fs::read_to_string(p).ok())
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }
    
    pub fn prompt_char(&self) -> &str {
        self.prompt_char.as_deref().unwrap_or("❯")
    }
}

