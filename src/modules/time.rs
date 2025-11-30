use crate::colors::{color, NEON_CYAN};
use chrono::Local;

pub fn render(format: &str) -> String {
    color(&Local::now().format(format).to_string(), NEON_CYAN)
}

