mod colors;
mod config;
mod modules;

use config::Config;
use std::env;

fn main() {
    let config = Config::load();
    let exit_code: i32 = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    
    let mut segments = Vec::new();
    
    if config.show_user {
        segments.push(modules::user());
    }
    
    if config.show_directory {
        segments.push(modules::directory());
    }
    
    if config.show_git {
        if let Some(git) = modules::git() {
            segments.push(git);
        }
    }
    
    let prompt_char = modules::character(exit_code == 0, config.prompt_char());
    
    // Format: segments arrow space (for user to type after)
    print!("{} {} ", segments.join(" "), prompt_char);
}

