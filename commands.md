# Shell Commands Reference

> Complete guide to all CLI commands, subcommands, and usage patterns for **Shell**.

---

## Table of Contents

- [Basic Usage](#basic-usage)
- [Core Commands](#core-commands)
- [Shell Integration](#shell-integration)
- [Configuration](#configuration)
- [Modules](#modules)
- [Debugging & Development](#debugging--development)

---

## Basic Usage

```bash
shell [OPTIONS] [EXIT_CODE] [DURATION_MS]
```

### Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `EXIT_CODE` | Exit code of the last command (0 = success) | `0` |
| `DURATION_MS` | Duration of last command in milliseconds | `0` |

### Examples

```bash
# Basic prompt (assumes success)
shell

# Pass last exit code
shell $?

# Pass exit code and duration
shell $? $DURATION_MS
```

---

## Core Commands

### `shell` (default)

Renders the prompt and prints it to stdout.

```bash
shell 0        # Render prompt with success status
shell 1        # Render prompt with error status (red prompt char)
```

---

### `shell init <SHELL>`

Generate shell-specific initialization script. Users can `eval` this to set up their prompt.

```bash
shell init bash
shell init zsh
shell init powershell
shell init fish
shell init nushell
```

#### Bash Output Example

```bash
# Outputs something like:
__shell_precmd() {
    local exit_code=$?
    local duration=$((EPOCHREALTIME - __shell_start))
    PS1="$(shell $exit_code $duration)"
}
__shell_preexec() {
    __shell_start=$EPOCHREALTIME
}
PROMPT_COMMAND="__shell_precmd"
trap '__shell_preexec' DEBUG
```

#### PowerShell Output Example

```powershell
# Outputs something like:
function global:prompt {
    $exitCode = $LASTEXITCODE
    $duration = if ($global:__shell_start) { 
        ((Get-Date) - $global:__shell_start).TotalMilliseconds 
    } else { 0 }
    shell $exitCode $duration
    $global:__shell_start = $null
}

Register-EngineEvent -SourceIdentifier PowerShell.OnIdle -Action {
    $global:__shell_start = Get-Date
} | Out-Null
```

#### Usage

```bash
# Add to ~/.bashrc
eval "$(shell init bash)"

# Add to ~/.zshrc
eval "$(shell init zsh)"

# Add to PowerShell $PROFILE
Invoke-Expression (shell init powershell | Out-String)
```

---

### `shell config`

Manage the configuration file (`~/.shell.toml`).

```bash
shell config              # Print current config location and status
shell config --print      # Print current configuration
shell config --default    # Print default configuration template
shell config --edit       # Open config in $EDITOR
shell config --path       # Print config file path only
```

#### Options

| Flag | Description |
|------|-------------|
| `--print` | Display current config as TOML |
| `--default` | Output a default config template |
| `--edit` | Open config file in `$EDITOR` or `notepad`/`nano` |
| `--path` | Print the config file path |
| `--validate` | Check config for errors |

#### Example Output (`--default`)

```toml
# ~/.shell.toml - Shell prompt configuration

# Show username@hostname
show_user = false

# Show current directory
show_directory = true

# Show git branch and status
show_git = true

# Show command duration (if > threshold)
show_duration = true
duration_threshold_ms = 2000

# Show current time
show_time = false
time_format = "%H:%M"

# Custom prompt character (default: ❯)
prompt_char = "❯"

# Modules to enable (order matters)
# modules = ["ssh", "user", "directory", "git", "venv", "lang", "duration", "time"]
```

---

### `shell explain`

Debug mode that shows what each segment represents.

```bash
shell explain
shell explain --verbose
shell explain --json
```

#### Example Output

```text
Prompt segments:
  [user]      rom@DESKTOP-ABC     (from $USERNAME + hostname)
  [directory] ~/dev/shell         (cwd with ~ substitution)
  [git]       main*              (branch: main, dirty: yes)
  [char]      ❯                   (success, exit_code: 0)

Render time: 1.8ms
```

#### JSON Output (`--json`)

```json
{
  "segments": [
    {"name": "user", "value": "rom@DESKTOP-ABC", "enabled": true},
    {"name": "directory", "value": "~/dev/shell", "enabled": true},
    {"name": "git", "value": "main*", "enabled": true, "branch": "main", "dirty": true},
    {"name": "char", "value": "❯", "success": true}
  ],
  "render_time_ms": 1.8,
  "config_path": "C:\\Users\\rom\\.shell.toml"
}
```

---

### `shell benchmark`

Measure prompt render performance.

```bash
shell benchmark
shell benchmark --iterations 100
shell benchmark --warmup 10
```

#### Options

| Flag | Description | Default |
|------|-------------|---------|
| `--iterations`, `-n` | Number of render iterations | `50` |
| `--warmup` | Warmup iterations (not counted) | `5` |
| `--detailed` | Show per-module timing | `false` |

#### Example Output

```text
Shell Benchmark Results
=======================
Iterations: 50
Warmup: 5

Total render time:
  Mean:   2.34ms
  Median: 2.21ms
  Min:    1.89ms
  Max:    4.12ms
  Std:    0.45ms

Module breakdown (--detailed):
  user:      0.12ms (5.1%)
  directory: 0.08ms (3.4%)
  git:       1.98ms (84.6%)
  character: 0.02ms (0.9%)
  overhead:  0.14ms (6.0%)

Status: ✓ Fast (< 10ms target)
```

---

### `shell version`

Display version information.

```bash
shell version
shell --version
shell -V
```

#### Output

```text
shell 0.1.0
rustc 1.75.0
git2 0.18.1
```

---

### `shell help`

Display help information.

```bash
shell help
shell --help
shell -h
shell help init       # Help for specific subcommand
shell help config
```

---

## Shell Integration

### Bash

```bash
# ~/.bashrc

# Simple (no duration tracking)
PS1='$(shell $?)'

# Full integration with duration
eval "$(shell init bash)"
```

### Zsh

```zsh
# ~/.zshrc

# Simple
precmd() { PROMPT="$(shell $?)" }

# Full integration with duration
eval "$(shell init zsh)"
```

### PowerShell

```powershell
# $PROFILE

# Simple
function prompt { shell $LASTEXITCODE }

# Full integration with duration
Invoke-Expression (shell init powershell | Out-String)
```

### Fish

```fish
# ~/.config/fish/config.fish

function fish_prompt
    shell $status
end

# Or with full integration
shell init fish | source
```

### Nushell

```nu
# config.nu

$env.PROMPT_COMMAND = { shell $env.LAST_EXIT_CODE }

# Or
shell init nushell | save -f ~/.cache/shell-init.nu
source ~/.cache/shell-init.nu
```

---

## Modules

Each module can be enabled/disabled in `~/.shell.toml`.

### Current Modules

| Module | Config Key | Description |
|--------|------------|-------------|
| `user` | `show_user` | Username and hostname (`user@host`) |
| `directory` | `show_directory` | Current working directory with `~` |
| `git` | `show_git` | Git branch and dirty status |
| `character` | (always on) | Prompt character with exit code color |

### Planned Modules

| Module | Config Key | Description |
|--------|------------|-------------|
| `duration` | `show_duration` | Last command execution time |
| `time` | `show_time` | Current time |
| `ssh` | `show_ssh` | SSH session indicator |
| `venv` | `show_venv` | Python virtual environment |
| `lang` | `show_lang` | Language runtime versions (Rust, Node, Python) |
| `aws` | `show_aws` | AWS profile name |
| `k8s` | `show_k8s` | Kubernetes context |
| `jobs` | `show_jobs` | Background job count |

---

## Debugging & Development

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SHELL_CONFIG` | Override config file path |
| `SHELL_LOG` | Enable debug logging (`trace`, `debug`, `info`) |
| `SHELL_NO_COLOR` | Disable colors (respects `NO_COLOR` standard) |

### Debug Mode

```bash
# Enable verbose logging
SHELL_LOG=debug shell

# Trace all operations
SHELL_LOG=trace shell explain --verbose
```

### Testing Prompts

```bash
# Test with different exit codes
shell 0      # Success (green)
shell 1      # Error (red)
shell 130    # Ctrl+C interrupted

# Test with duration
shell 0 5000   # 5 second duration

# Test in different directories
cd /tmp && shell
cd ~ && shell
cd ~/some/git/repo && shell
```

---

## Exit Codes

Shell itself returns these exit codes:

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `3` | Config parse error |

---

## Future Commands

### `shell preset <NAME>`

Apply a preset theme/configuration.

```bash
shell preset list              # List available presets
shell preset apply minimal     # Apply minimal preset
shell preset apply powerline   # Apply powerline-style preset
shell preset apply neon        # Apply neon theme (default)
```

### `shell module <ACTION>`

Manage individual modules.

```bash
shell module list              # List all available modules
shell module enable git        # Enable git module
shell module disable user      # Disable user module
shell module info git          # Show module details
```

### `shell cache`

Manage prompt caching (for slow modules).

```bash
shell cache clear              # Clear all cached data
shell cache show               # Show cache statistics
```

---

## Quick Reference

```bash
# Basic usage
shell [exit_code] [duration_ms]

# Setup
shell init bash|zsh|powershell|fish

# Configuration
shell config --print
shell config --edit
shell config --default > ~/.shell.toml

# Debugging
shell explain
shell benchmark
SHELL_LOG=debug shell

# Help
shell --help
shell help <command>
shell --version
```

