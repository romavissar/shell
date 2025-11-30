use crate::colors::{color, NEON_BLUE};
use std::{env, process::Command};

pub fn render() -> Option<String> {
    let cwd = env::current_dir().ok()?;
    if cwd.join("Cargo.toml").exists() {
        return version("rustc", &["--version"], "🦀", 1);
    }
    if cwd.join("package.json").exists() || cwd.join("node_modules").exists() {
        return version("node", &["--version"], "⬢", 0);
    }
    if cwd.join("pyproject.toml").exists() || cwd.join("requirements.txt").exists() || 
       cwd.join("setup.py").exists() || cwd.join(".python-version").exists() {
        return version("python", &["--version"], "🐍", 1);
    }
    if cwd.join("go.mod").exists() {
        return version("go", &["version"], "🐹", 2);
    }
    None
}

fn version(cmd: &str, args: &[&str], icon: &str, idx: usize) -> Option<String> {
    Command::new(cmd).args(args).output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().nth(idx).map(|v| v.trim_start_matches('v').to_string()))
        .map(|v| color(&format!("{}{}", icon, v), NEON_BLUE))
}

