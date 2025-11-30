use serde::Deserialize;
use std::fs;

fn t() -> bool { true }
fn threshold() -> u64 { 2000 }
fn time_fmt() -> String { "%H:%M".into() }

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)] pub show_user: bool,
    #[serde(default = "t")] pub show_directory: bool,
    #[serde(default = "t")] pub show_git: bool,
    #[serde(default = "t")] pub show_ssh: bool,
    #[serde(default = "t")] pub show_venv: bool,
    #[serde(default)] pub show_lang: bool,
    #[serde(default = "t")] pub show_duration: bool,
    #[serde(default)] pub show_time: bool,
    #[serde(default = "threshold")] pub duration_threshold_ms: u64,
    #[serde(default = "time_fmt")] pub time_format: String,
    #[serde(default)] pub prompt_char: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { show_user: false, show_directory: true, show_git: true, show_ssh: true, show_venv: true, show_lang: false, show_duration: true, show_time: false, duration_threshold_ms: 2000, time_format: "%H:%M".into(), prompt_char: None }
    }
}

impl Config {
    pub fn load() -> Self {
        dirs::home_dir().map(|h| h.join(".shell.toml")).and_then(|p| fs::read_to_string(p).ok()).and_then(|s| toml::from_str(&s).ok()).unwrap_or_default()
    }
    pub fn prompt_char(&self) -> &str { self.prompt_char.as_deref().unwrap_or("❯") }
}
