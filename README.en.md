# MicuSubCodeLine

[English](README.en.md) | [中文](README.md)

A high-performance Claude Code statusline tool written in Rust with Git integration, usage tracking, Sub2API subscription display, and Claude Code enhancement utilities.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)

> ⚠️ **v2.0.0 — Breaking change.** The statusline now renders a **fixed, locked 3-line layout** that **cannot** be altered by any config file, theme file, `--theme` flag or TUI edit. Subscription data is fetched from **`https://sub.micuapi.ai`**. If you relied on a custom `config.toml`/theme in v1.x, it is no longer applied to the rendered statusline.

## Screenshots

![MicuSubCodeLine](assets/展示图.png)

The locked statusline always renders three lines:

- **Line 1** — `模型`(Model) · `上下文`(Context) · `⌥ branch` · `(+N,−N)` git changes · `合计`(Total t/s)
- **Line 2** — `会话`(Session) · `费用`(Cost) · `cwd` working directory
- **Line 3** — Plan name · `每日`(Daily $used/$limit) · `每周`(Weekly $used/$limit) · `到期`(Days to expiry)

## Features

### Core Functionality
- **Fixed locked layout** — pixel-stable 3-line powerline output, not user-configurable
- **Git integration** with branch and working-tree change counts
- **Model display** with simplified Claude model names
- **Context / throughput** based on transcript analysis (tokens, t/s)
- **Subscription display** real-time Sub2API info from `https://sub.micuapi.ai` (auto-reads API Key from Claude Code settings, zero config)

### Claude Code Enhancement
- **Context warning disabler** - Remove annoying "Context low" messages
- **Verbose mode enabler** - Enhanced output detail
- **Robust patcher** - Survives Claude Code version updates
- **Automatic backups** - Safe modification with easy recovery

## Installation

### Download Pre-built Binary

Download from [Releases](https://github.com/zuoliangyu/MicuSubCodeLine/releases):

#### Linux (Dynamic)
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-linux-x64.tar.gz
tar -xzf micusubcodeline-linux-x64.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```
*Requires: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### Linux (Static)
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-linux-x64-static.tar.gz
tar -xzf micusubcodeline-linux-x64-static.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```
*Works on any Linux distribution (static, no dependencies)*

#### macOS (Intel)
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-macos-x64.tar.gz
tar -xzf micusubcodeline-macos-x64.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```

#### macOS (Apple Silicon)
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-macos-arm64.tar.gz
tar -xzf micusubcodeline-macos-arm64.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```

#### Windows
```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\micusubcodeline"
Invoke-WebRequest -Uri "https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-windows-x64.zip" -OutFile "micusubcodeline-windows-x64.zip"
Expand-Archive -Path "micusubcodeline-windows-x64.zip" -DestinationPath "."
Move-Item "micusubcodeline.exe" "$env:USERPROFILE\.claude\micusubcodeline\"
```

### Build from Source

```bash
git clone https://github.com/zuoliangyu/MicuSubCodeLine.git
cd MicuSubCodeLine
cargo build --release

# Linux/macOS
mkdir -p ~/.claude/micusubcodeline
cp target/release/micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\micusubcodeline"
copy target\release\micusubcodeline.exe "$env:USERPROFILE\.claude\micusubcodeline\"
```

### Claude Code Configuration

Add to your Claude Code `settings.json`:

**Linux/macOS:**
```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micusubcodeline/micusubcodeline",
    "padding": 0
  }
}
```

**Windows:**
```json
{
  "statusLine": {
    "type": "command",
    "command": "%USERPROFILE%\\.claude\\micusubcodeline\\micusubcodeline.exe",
    "padding": 0
  }
}
```

## Usage

### Subscription Info (Zero Config)

The subscription line automatically reads your API Key from Claude Code's settings — no manual configuration needed. Data is fetched from `https://sub.micuapi.ai/v1/usage`.

Reading priority:
1. `~/.claude/settings.local.json` → `env.ANTHROPIC_API_KEY` / `ANTHROPIC_AUTH_TOKEN`
2. `~/.claude/settings.json` → `env.ANTHROPIC_API_KEY` / `ANTHROPIC_AUTH_TOKEN`
3. Environment variable `ANTHROPIC_API_KEY` / `ANTHROPIC_AUTH_TOKEN`
4. `~/.claude/micusubcodeline/subscription_config.txt` (legacy fallback)

```bash
# Check API Key detection status
micusubcodeline --init-subscription
```

### Claude Code Enhancement

```bash
# Disable context warnings and enable verbose mode
micusubcodeline --patch /path/to/claude-code/cli.js
```

### Legacy commands (no effect on the locked statusline)

Since v2.0.0 the rendered statusline is hard-locked. The following commands still
exist for compatibility but **do not change the displayed statusline**:

```bash
micusubcodeline --init      # writes a config.toml that is ignored at render time
micusubcodeline --check     # validates that (ignored) config file
micusubcodeline --print     # prints the (ignored) config
micusubcodeline --config    # opens the TUI; edits are not applied to output
micusubcodeline --theme X   # theme override is not applied to output
```

## Requirements

- **Git**: Version 1.5+ (Git 2.22+ recommended for better branch detection)
- **Terminal**: Must support Nerd Fonts for proper powerline/icon display
  - Install a [Nerd Font](https://www.nerdfonts.com/) (e.g., FiraCode Nerd Font, JetBrains Mono Nerd Font)
  - Chinese users: [Maple Font](https://github.com/subframe7536/maple-font) (Nerd Font with CJK support) is recommended
  - Configure your terminal to use the Nerd Font
- **Claude Code**: For statusline integration

## Development

```bash
# Build development version
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the [MIT License](LICENSE).
