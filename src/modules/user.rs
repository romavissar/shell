use crate::colors::{bold, NEON_PINK};
use std::env;

pub fn render() -> String {
    let user = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "user".into());
    
    let host = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "host".into());
    
    bold(&format!("{}@{}", user, host), NEON_PINK)
}

