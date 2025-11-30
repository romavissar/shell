use crate::colors::{color, NEON_YELLOW};

pub fn render(duration_ms: u64, threshold: u64) -> Option<String> {
    if duration_ms < threshold { return None; }
    let s = duration_ms / 1000;
    let display = if s >= 3600 { format!("{}h{}m", s / 3600, (s % 3600) / 60) }
                  else if s >= 60 { format!("{}m{}s", s / 60, s % 60) }
                  else { format!("{}.{}s", s, (duration_ms % 1000) / 100) };
    Some(color(&format!("⏱{}", display), NEON_YELLOW))
}

