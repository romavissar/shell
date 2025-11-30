use crate::colors::{color, NEON_PURPLE};
use std::{env, path::Path};

pub fn render() -> Option<String> {
    env::var("VIRTUAL_ENV").ok()
        .and_then(|p| Path::new(&p).file_name()?.to_str().map(String::from))
        .or_else(|| env::var("CONDA_DEFAULT_ENV").ok())
        .map(|name| color(&format!("({})", name), NEON_PURPLE))
}

