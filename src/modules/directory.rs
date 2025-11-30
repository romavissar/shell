use crate::colors::{bold, color, NEON_PINK, WHITE};
use std::{env, path::Path};

pub fn render() -> String {
    let cwd = env::current_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|_| "?".into());
    let home = dirs::home_dir().map(|h| h.to_string_lossy().into_owned());
    
    let (in_home, rest) = home.as_ref().map(|h| {
        if cwd == *h { (true, String::new()) }
        else { cwd.strip_prefix(&format!("{h}\\")).or_else(|| cwd.strip_prefix(&format!("{h}/"))).map(|r| (true, r.to_string())).unwrap_or((false, String::new())) }
    }).unwrap_or((false, String::new()));

    let short = |p: &str| -> String {
        let c: Vec<_> = Path::new(p).components().collect();
        if c.len() <= 2 { p.replace('\\', "/") } else { format!("{}/{}", c[c.len()-2].as_os_str().to_string_lossy(), c[c.len()-1].as_os_str().to_string_lossy()) }
    };

    if in_home { if rest.is_empty() { bold("~", NEON_PINK) } else { format!("{} {}", bold("~", NEON_PINK), color(&short(&rest), WHITE)) } }
    else { color(&short(&cwd), WHITE) }
}
