use crate::colors::{bold, NEON_GREEN, NEON_RED};

pub fn render(success: bool, char: &str) -> String {
    let color = if success { NEON_GREEN } else { NEON_RED };
    bold(char, color)
}

