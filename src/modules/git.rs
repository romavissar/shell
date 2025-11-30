use crate::colors::{color, NEON_PURPLE, NEON_YELLOW};
use git2::Repository;
use std::env;

pub fn render() -> Option<String> {
    let repo = Repository::discover(env::current_dir().ok()?).ok()?;
    let head = repo.head().ok()?;
    let branch = head.shorthand().unwrap_or("HEAD");
    
    let dirty = repo.statuses(None)
        .map(|s| s.iter().any(|e| e.status() != git2::Status::CURRENT))
        .unwrap_or(false);
    
    let marker = if dirty { "*" } else { "" };
    
    Some(format!(
        "{}{}{}",
        color("", NEON_PURPLE),
        color(&format!("{}{}", branch, marker), NEON_YELLOW),
        color("", NEON_PURPLE)
    ))
}

