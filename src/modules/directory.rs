use crate::colors::{bold, color, NEON_PINK, WHITE};
use std::env;
use std::path::Path;

pub fn render() -> String {
    let cwd = env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "?".into());
    
    let home = dirs::home_dir().map(|h| h.to_string_lossy().into_owned());
    
    // Check if we're in home or under home
    let (is_home_relative, path_after_home) = match &home {
        Some(h) => {
            if cwd == *h {
                (true, String::new())
            } else if let Some(rest) = cwd.strip_prefix(&format!("{}\\", h))
                .or_else(|| cwd.strip_prefix(&format!("{}/", h))) {
                (true, rest.to_string())
            } else {
                (false, String::new())
            }
        }
        None => (false, String::new()),
    };
    
    if is_home_relative {
        if path_after_home.is_empty() {
            // We're in home directory - just bold pink tilde
            bold("~", NEON_PINK)
        } else {
            // Get last two path components
            let path = Path::new(&path_after_home);
            let components: Vec<_> = path.components().collect();
            let short_path = if components.len() <= 2 {
                path_after_home.replace('\\', "/")
            } else {
                let last_two: Vec<_> = components.iter().rev().take(2).collect();
                format!("{}/{}", 
                    last_two[1].as_os_str().to_string_lossy(),
                    last_two[0].as_os_str().to_string_lossy())
            };
            // Bold pink tilde + space + white path
            format!("{} {}", bold("~", NEON_PINK), color(&short_path, WHITE))
        }
    } else {
        // Not under home, show last two components in white
        let path = Path::new(&cwd);
        let components: Vec<_> = path.components().collect();
        let short_path = if components.len() <= 2 {
            cwd.replace('\\', "/")
        } else {
            let last_two: Vec<_> = components.iter().rev().take(2).collect();
            format!("{}/{}", 
                last_two[1].as_os_str().to_string_lossy(),
                last_two[0].as_os_str().to_string_lossy())
        };
        color(&short_path, WHITE)
    }
}

