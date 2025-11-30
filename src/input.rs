use crossterm::{cursor, event::{self, Event, KeyCode, KeyModifiers}, terminal, ExecutableCommand};
use std::{collections::HashMap, fs, io::{self, Write}};

const DIM: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";
const PINK: &str = "\x1b[38;5;198m";
const WHITE: &str = "\x1b[38;5;255m";
const CYAN: &str = "\x1b[38;5;51m";
const GREEN: &str = "\x1b[38;5;118m";
const YELLOW: &str = "\x1b[38;5;226m";

pub fn readline() -> io::Result<String> {
    let history = load_history_ranked();
    let mut input = String::new();
    let mut stdout = io::stdout();
    
    terminal::enable_raw_mode()?;
    
    loop {
        let prediction = get_prediction(&input, &history);
        
        stdout.execute(cursor::MoveToColumn(0))?;
        print!("\x1b[K{}", colorize(&input));
        if let Some(ref p) = prediction {
            print!("{}{}{}", DIM, p, RESET);
            stdout.execute(cursor::MoveToColumn(input.len() as u16))?;
        }
        stdout.flush()?;
        
        if let Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (KeyCode::Enter, _) => break,
                (KeyCode::Tab, _) if prediction.is_some() => input.push_str(&prediction.unwrap()),
                (KeyCode::Right, _) if prediction.is_some() => input.push_str(&prediction.unwrap()),
                (KeyCode::Backspace, _) => { input.pop(); }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode()?;
                    println!();
                    std::process::exit(130);
                }
                (KeyCode::Char(c), _) => input.push(c),
                _ => {}
            }
        }
    }
    
    terminal::disable_raw_mode()?;
    println!();
    
    if !input.trim().is_empty() { save_to_history(&input); }
    Ok(input)
}

fn colorize(input: &str) -> String {
    if input.is_empty() { return String::new(); }
    
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut in_command = true;
    let mut in_string = false;
    let mut string_char = '"';
    
    while let Some(c) = chars.next() {
        match c {
            // String literals
            '"' | '\'' if !in_string => {
                in_string = true;
                string_char = c;
                in_command = false;
                result.push_str(GREEN);
                result.push(c);
            }
            c if in_string && c == string_char => {
                result.push(c);
                result.push_str(RESET);
                in_string = false;
            }
            // Pipes and operators - next word is a command
            '|' | '&' | ';' => {
                result.push_str(YELLOW);
                result.push(c);
                result.push_str(RESET);
                in_command = true;
            }
            // Flags
            '-' if !in_string && !in_command => {
                result.push_str(CYAN);
                result.push(c);
                // Consume rest of flag
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '-' {
                        result.push(chars.next().unwrap());
                    } else { break; }
                }
                result.push_str(RESET);
            }
            // Whitespace ends command word
            ' ' | '\t' if in_command && !in_string => {
                result.push_str(RESET);
                result.push(c);
                in_command = false;
            }
            // Regular chars
            _ => {
                if in_command && !in_string {
                    result.push_str(PINK);
                } else if !in_string {
                    result.push_str(WHITE);
                }
                result.push(c);
            }
        }
    }
    result.push_str(RESET);
    result
}

fn get_prediction(input: &str, history: &[(String, f64)]) -> Option<String> {
    if input.is_empty() { return None; }
    
    history.iter()
        .filter(|(cmd, _)| cmd.starts_with(input) && cmd.len() > input.len())
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(cmd, _)| cmd[input.len()..].to_string())
}

fn history_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".shell_history"))
}

fn load_history_ranked() -> Vec<(String, f64)> {
    let lines: Vec<String> = history_path()
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|s| s.lines().map(String::from).collect())
        .unwrap_or_default();
    
    let total = lines.len();
    if total == 0 { return Vec::new(); }
    
    let mut freq: HashMap<String, usize> = HashMap::new();
    let mut recency: HashMap<String, usize> = HashMap::new();
    
    for (i, cmd) in lines.iter().enumerate() {
        *freq.entry(cmd.clone()).or_insert(0) += 1;
        recency.insert(cmd.clone(), i);
    }
    
    let max_freq = *freq.values().max().unwrap_or(&1) as f64;
    
    freq.into_iter()
        .map(|(cmd, count)| {
            let freq_score = count as f64 / max_freq;
            let recency_score = recency.get(&cmd).copied().unwrap_or(0) as f64 / total as f64;
            let score = freq_score * 0.4 + recency_score * 0.6;
            (cmd, score)
        })
        .collect()
}

fn save_to_history(cmd: &str) {
    if let Some(path) = history_path() {
        let mut history = history_path()
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_default();
        history.push_str(cmd.trim());
        history.push('\n');
        let _ = fs::write(path, history);
    }
}
