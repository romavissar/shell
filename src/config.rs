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
        let path = match dirs::home_dir() {
            Some(h) => h.join(".shell.toml"),
            None => return Self::default(),
        };
        
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(_) => return Self::default(),
        };
        
        // Handle different encodings (UTF-16 LE, UTF-16 BE, UTF-8 with BOM, UTF-8)
        let content = if bytes.starts_with(&[0xFF, 0xFE]) {
            // UTF-16 LE (common Windows encoding)
            let u16s: Vec<u16> = bytes[2..].chunks(2)
                .filter_map(|c| if c.len() == 2 { Some(u16::from_le_bytes([c[0], c[1]])) } else { None })
                .collect();
            String::from_utf16_lossy(&u16s)
        } else if bytes.starts_with(&[0xFE, 0xFF]) {
            // UTF-16 BE
            let u16s: Vec<u16> = bytes[2..].chunks(2)
                .filter_map(|c| if c.len() == 2 { Some(u16::from_be_bytes([c[0], c[1]])) } else { None })
                .collect();
            String::from_utf16_lossy(&u16s)
        } else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            // UTF-8 with BOM
            String::from_utf8_lossy(&bytes[3..]).into_owned()
        } else {
            // Plain UTF-8
            String::from_utf8_lossy(&bytes).into_owned()
        };
        
        toml::from_str(&content).unwrap_or_default()
    }
    
    pub fn prompt_char(&self) -> &str { self.prompt_char.as_deref().unwrap_or("❯") }
}
