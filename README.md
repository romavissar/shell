# BUILD IDEA

# ⚡️ Shell — Fast. Colorful. Cross-Platform.

> A lightning-fast, neon-powered shell prompt built with love in [Cursor](https://cursor.sh).  
> Designed for hackers who care about **speed**, **style**, and **clarity**.

---

## ✨ Overview

**Shell** is a next-generation terminal prompt that brings speed and beauty to your command line.  
It's inspired by [Starship.rs](https://starship.rs) — minimal, fast, and endlessly customizable —  
but reimagined with **neon aesthetics**, **snappy performance**, and **zero startup lag**.

Shell is **not a shell replacement** (like Bash or PowerShell).  
It's a **universal prompt generator** that integrates with *any* shell, instantly upgrading your terminal experience.

---

## 🚀 Features

- ⚡ **Instant render time** — written in **Rust** for maximum performance  
- 🌈 **Vibrant neon UI** — glowing ANSI colors designed for dark terminals  
- 🧠 **Smart prompt logic** — only shows what's relevant (Git, status, path, etc.)  
- 🔧 **Easy configuration** — customize via a simple `~/.shell.toml` file  
- 💻 **Cross-platform** — works on macOS, Windows, and Linux  
- 🧩 **Modular design** — plug in or disable modules as you wish  

---

## 🎨 Example Prompt

```text
[](neon-green)[user@host](bold-neon-pink)[](neon-blue)[~/dev/shell](neon-cyan)[](reset) λ
```

---

## 🛠️ Tech Stack

**Shell** is built with a carefully curated set of modern, high-performance technologies:

### Core
- **[Rust](https://www.rust-lang.org/)** — blazing-fast compiled language with zero-cost abstractions
- **[Cargo](https://doc.rust-lang.org/cargo/)** — Rust's package manager and build system

### Key Dependencies
- **[crossterm](https://crates.io/crates/crossterm)** — cross-platform terminal manipulation (colors, cursor control)
- **[git2](https://crates.io/crates/git2)** — libgit2 bindings for Git repository info
- **[toml](https://crates.io/crates/toml)** — configuration file parsing
- **[clap](https://crates.io/crates/clap)** — command-line argument parsing
- **[dirs](https://crates.io/crates/dirs)** — cross-platform directory paths (home, config, etc.)
- **[serde](https://crates.io/crates/serde)** — serialization/deserialization framework

### Development Tools
- **rustfmt** — code formatting
- **clippy** — linting and best practices
- **cargo-watch** — automatic rebuild on file changes

---

## 🏗️ Architecture

**Shell** follows a modular, plugin-based architecture for maximum flexibility and maintainability.

### Project Structure

```text
shell/
├── src/
│   ├── main.rs              # Entry point, orchestrates prompt rendering
│   ├── config.rs            # Configuration loading (~/.shell.toml)
│   ├── colors.rs            # ANSI color utilities and neon theme
│   └── modules/
│       ├── mod.rs           # Module exports
│       ├── user.rs          # Username/hostname display
│       ├── directory.rs     # Current directory with ~ shortening
│       ├── git.rs           # Git branch and dirty status
│       └── character.rs     # Prompt character (λ) with exit code color
├── Cargo.toml               # Dependencies
└── README.md
```

### Component Responsibilities

#### **`main.rs`**
- Loads config, reads exit code from CLI arg
- Calls enabled modules and joins output
- Prints final prompt

#### **`config.rs`**
- Loads `~/.shell.toml` with serde
- Provides defaults for missing options

#### **`colors.rs`**
- Neon color palette (green, pink, blue, cyan, purple, yellow, red)
- `color()` and `bold()` helper functions

#### **`modules/`**
- **`user.rs`** — `user@host` with neon pink
- **`directory.rs`** — current path with `~` shortening
- **`git.rs`** — branch name + `*` if dirty
- **`character.rs`** — `λ` (green = success, red = error)

---

## 📋 Installation

```bash
cargo install --path .
```

## ⚙️ Configuration

Create `~/.shell.toml`:

```toml
show_user = true
show_directory = true
show_git = true
prompt_char = "λ"
```

## 🔌 Shell Integration

**Bash** (`~/.bashrc`):
```bash
eval "$(shell init bash)"
# or manually:
PS1='$(shell $?)'
```

**PowerShell** (`$PROFILE`):
```powershell
function prompt { shell $LASTEXITCODE }
```

**Zsh** (`~/.zshrc`):
```zsh
precmd() { PROMPT="$(shell $?)" }
```

---

## 📄 License

MIT
