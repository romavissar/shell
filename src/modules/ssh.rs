use crate::colors::{bold, NEON_RED};
use std::env;

pub fn render() -> Option<String> {
    if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_CLIENT").is_ok() {
        Some(bold("⚡SSH", NEON_RED))
    } else { None }
}

