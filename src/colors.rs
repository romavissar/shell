/// ANSI color codes for neon terminal aesthetics
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

// Neon palette
#[allow(dead_code)]
pub const NEON_GREEN: &str = "\x1b[38;5;118m";
#[allow(dead_code)]
pub const NEON_PINK: &str = "\x1b[38;5;198m";
#[allow(dead_code)]
pub const NEON_BLUE: &str = "\x1b[38;5;39m";
#[allow(dead_code)]
pub const NEON_CYAN: &str = "\x1b[38;5;51m";
#[allow(dead_code)]
pub const NEON_YELLOW: &str = "\x1b[38;5;226m";
#[allow(dead_code)]
pub const NEON_RED: &str = "\x1b[38;5;196m";
#[allow(dead_code)]
pub const NEON_PURPLE: &str = "\x1b[38;5;141m";
#[allow(dead_code)]
pub const WHITE: &str = "\x1b[38;5;255m";

#[inline]
pub fn color(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, RESET)
}

#[inline]
pub fn bold(text: &str, color: &str) -> String {
    format!("{}{}{}{}", BOLD, color, text, RESET)
}

