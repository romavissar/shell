mod colors;
mod config;
mod input;
mod modules;

use config::Config;
use std::{env, time::Instant};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("init") => init(args.get(1).map(|s| s.as_str()).unwrap_or("bash")),
        Some("config") => cmd_config(args.get(1).map(|s| s.as_str())),
        Some("explain") => explain(),
        Some("benchmark") => benchmark(args.get(1).and_then(|s| s.parse().ok()).unwrap_or(50)),
        Some("version" | "-V" | "--version") => println!("shell {VERSION}"),
        Some("help" | "-h" | "--help") => print_help(),
        Some("input") => { input::readline().map(|c| print!("{c}")).ok(); }
        _ => prompt(args.first().and_then(|s| s.parse().ok()).unwrap_or(0),
                    args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0)),
    }
}

fn prompt(exit_code: i32, duration_ms: u64) {
    let c = Config::load();
    let mut s = Vec::new();
    if c.show_ssh { if let Some(x) = modules::ssh() { s.push(x); } }
    if c.show_user { s.push(modules::user()); }
    if c.show_directory { s.push(modules::directory()); }
    if c.show_git { if let Some(x) = modules::git() { s.push(x); } }
    if c.show_venv { if let Some(x) = modules::venv() { s.push(x); } }
    if c.show_lang { if let Some(x) = modules::lang() { s.push(x); } }
    if c.show_duration { if let Some(x) = modules::duration(duration_ms, c.duration_threshold_ms) { s.push(x); } }
    if c.show_time { s.push(modules::time(&c.time_format)); }
    print!("{} {} ", s.join(" "), modules::character(exit_code == 0, c.prompt_char()));
}

fn init(shell: &str) {
    print!("{}", match shell {
        "bash" => r#"__shell_cmd_start() { __shell_start=$EPOCHREALTIME; }
__shell_cmd_end() { local e=$?; local d=0; [[ -n $__shell_start ]] && d=$(echo "($EPOCHREALTIME - $__shell_start) * 1000" | bc | cut -d. -f1); PS1="$(shell $e $d)"; unset __shell_start; }
trap '__shell_cmd_start' DEBUG
PROMPT_COMMAND="__shell_cmd_end""#,
        "zsh" => r#"autoload -Uz add-zsh-hook
__shell_preexec() { __shell_start=$EPOCHREALTIME }
__shell_precmd() { local e=$?; local d=0; [[ -n $__shell_start ]] && d=$(( ($EPOCHREALTIME - $__shell_start) * 1000 )); PROMPT="$(shell $e ${d%.*})"; unset __shell_start }
add-zsh-hook preexec __shell_preexec
add-zsh-hook precmd __shell_precmd"#,
        "powershell" | "pwsh" => r#"$global:__ShellStart = $null
Set-PSReadLineKeyHandler -Chord Enter -ScriptBlock {
    $global:__ShellStart = Get-Date
    [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
}
function global:prompt {
    $e = $LASTEXITCODE; $d = 0
    if ($global:__ShellStart) { $d = [int]((Get-Date) - $global:__ShellStart).TotalMilliseconds }
    $global:__ShellStart = $null
    shell $e $d
}"#,
        "fish" => r#"function fish_prompt
    set -l e $status
    set -l d 0
    if set -q __shell_start; set d (math "($EPOCHREALTIME - $__shell_start) * 1000" | string split .)[1]; end
    set -e __shell_start
    shell $e $d
end
function __shell_preexec --on-event fish_preexec; set -g __shell_start $EPOCHREALTIME; end"#,
        "nu" | "nushell" => r#"$env.PROMPT_COMMAND = {|| shell $env.LAST_EXIT_CODE 0 }"#,
        _ => "echo 'Unknown shell. Supported: bash, zsh, powershell, fish, nushell'"
    });
}

fn cmd_config(flag: Option<&str>) {
    let path = dirs::home_dir().map(|h| h.join(".shell.toml")).unwrap();
    match flag {
        Some("--path") => println!("{}", path.display()),
        Some("--edit") => { let _ = std::process::Command::new(env::var("EDITOR").unwrap_or_else(|_| 
            if cfg!(windows) { "notepad".into() } else { "nano".into() })).arg(&path).status(); }
        Some("--default") => print!("{}", DEFAULT_CONFIG),
        Some("--print") => print!("{}", std::fs::read_to_string(&path).unwrap_or_else(|_| "# No config found\n".into())),
        _ => println!("Config: {}\nExists: {}\n\nFlags: --path, --print, --default, --edit", path.display(), path.exists()),
    }
}

fn explain() {
    let c = Config::load();
    let t = Instant::now();
    println!("Prompt segments:");
    if c.show_ssh { println!("  [ssh]       {:?}", modules::ssh()); }
    if c.show_user { println!("  [user]      {}", strip_ansi(&modules::user())); }
    if c.show_directory { println!("  [directory] {}", strip_ansi(&modules::directory())); }
    if c.show_git { println!("  [git]       {:?}", modules::git().map(|s| strip_ansi(&s))); }
    if c.show_venv { println!("  [venv]      {:?}", modules::venv().map(|s| strip_ansi(&s))); }
    if c.show_lang { println!("  [lang]      {:?}", modules::lang().map(|s| strip_ansi(&s))); }
    if c.show_duration { println!("  [duration]  (requires duration_ms arg)"); }
    if c.show_time { println!("  [time]      {}", strip_ansi(&modules::time(&c.time_format))); }
    println!("  [char]      {}", strip_ansi(&modules::character(true, c.prompt_char())));
    println!("\nRender time: {:.2}ms", t.elapsed().as_secs_f64() * 1000.0);
}

fn benchmark(n: u32) {
    let c = Config::load();
    print!("Warming up...");
    for _ in 0..5 { let _ = render_prompt(&c, 0, 0); }
    println!(" done\nRunning {n} iterations...");
    let times: Vec<_> = (0..n).map(|_| { let t = Instant::now(); render_prompt(&c, 0, 0); t.elapsed().as_secs_f64() * 1000.0 }).collect();
    let (sum, min, max) = (times.iter().sum::<f64>(), times.iter().cloned().fold(f64::MAX, f64::min), times.iter().cloned().fold(0.0, f64::max));
    let mean = sum / n as f64;
    let mut sorted = times.clone(); sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("\nResults:\n  Mean:   {mean:.3}ms\n  Median: {:.3}ms\n  Min:    {min:.3}ms\n  Max:    {max:.3}ms", sorted[n as usize / 2]);
    println!("\nStatus: {}", if mean < 10.0 { "✓ Fast" } else if mean < 50.0 { "⚠ Acceptable" } else { "✗ Slow" });
}

fn render_prompt(c: &Config, exit_code: i32, duration_ms: u64) -> String {
    let mut s = Vec::new();
    if c.show_ssh { if let Some(x) = modules::ssh() { s.push(x); } }
    if c.show_user { s.push(modules::user()); }
    if c.show_directory { s.push(modules::directory()); }
    if c.show_git { if let Some(x) = modules::git() { s.push(x); } }
    if c.show_venv { if let Some(x) = modules::venv() { s.push(x); } }
    if c.show_lang { if let Some(x) = modules::lang() { s.push(x); } }
    if c.show_duration { if let Some(x) = modules::duration(duration_ms, c.duration_threshold_ms) { s.push(x); } }
    if c.show_time { s.push(modules::time(&c.time_format)); }
    format!("{} {} ", s.join(" "), modules::character(exit_code == 0, c.prompt_char()))
}

fn strip_ansi(s: &str) -> String { 
    let mut out = String::new(); let mut esc = false;
    for c in s.chars() { if c == '\x1b' { esc = true; } else if esc && c == 'm' { esc = false; } else if !esc { out.push(c); } }
    out
}

fn print_help() {
    println!(r#"shell {VERSION} - Fast, colorful shell prompt

USAGE: shell [COMMAND] [OPTIONS]

COMMANDS:
  <exit_code> [duration_ms]  Render prompt (default)
  init <shell>               Output shell init script (bash/zsh/powershell/fish/nushell)
  config [--flag]            Manage config (--path, --print, --default, --edit)
  explain                    Show prompt segment breakdown
  benchmark [n]              Benchmark prompt rendering (default: 50 iterations)
  input                      Interactive readline with autocomplete
  version, -V                Show version
  help, -h                   Show this help

EXAMPLES:
  eval "$(shell init bash)"  # Add to ~/.bashrc
  shell config --default > ~/.shell.toml
  shell 0 1500               # Render with exit=0, duration=1.5s"#);
}

const DEFAULT_CONFIG: &str = r#"# Shell prompt configuration (~/.shell.toml)

show_user = false
show_directory = true
show_git = true
show_ssh = true
show_venv = true
show_lang = false
show_duration = true
show_time = false

duration_threshold_ms = 2000
time_format = "%H:%M"
prompt_char = "❯"
"#;
