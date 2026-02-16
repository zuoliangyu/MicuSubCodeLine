# MicuSubCodeLine

[English](README.md) | [中文](README.zh.md)

基于 Rust 的高性能 Claude Code 状态栏工具，集成 Git 信息、使用量跟踪、交互式 TUI 配置、Sub2API 订阅信息显示和 Claude Code 补丁工具。

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)

## 截图

![MicuSubCodeLine](assets/img1.png)

状态栏显示：模型 | 目录 | Git 分支状态 | 上下文窗口信息 | 订阅信息

## 特性

### 核心功能
- **Git 集成** 显示分支、状态和跟踪信息
- **模型显示** 简化的 Claude 模型名称
- **使用量跟踪** 基于转录文件分析
- **目录显示** 显示当前工作空间
- **订阅信息** 实时显示 Sub2API 订阅状态
- **简洁设计** 使用 Nerd Font 图标

### 交互式 TUI 功能
- **交互式主菜单** 无输入时直接执行显示菜单
- **TUI 配置界面** 实时预览配置效果
- **主题系统** 多种内置预设主题
- **段落自定义** 精细化控制各段落
- **配置管理** 初始化、检查、编辑配置

### Claude Code 增强
- **禁用上下文警告** 移除烦人的"Context low"消息
- **启用详细模式** 增强输出详细信息
- **稳定补丁器** 适应 Claude Code 版本更新
- **自动备份** 安全修改，支持轻松恢复

## 安装

### 下载预编译二进制

从 [Releases](https://github.com/zuoliangyu/MicuSubCodeLine/releases) 下载：

#### Linux（动态链接版本）
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-linux-x64.tar.gz
tar -xzf micusubcodeline-linux-x64.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```
*系统要求: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### Linux（静态链接版本）
```bash
mkdir -p ~/.claude/micusubcodeline
wget https://github.com/zuoliangyu/MicuSubCodeLine/releases/latest/download/micusubcodeline-linux-x64-static.tar.gz
tar -xzf micusubcodeline-linux-x64-static.tar.gz
cp micusubcodeline ~/.claude/micusubcodeline/
chmod +x ~/.claude/micusubcodeline/micusubcodeline
```
*适用于任何 Linux 发行版（静态链接，无依赖）*

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

### 从源码构建

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

### Claude Code 配置

添加到 Claude Code `settings.json`：

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

## 使用

### 配置管理

```bash
# 初始化配置文件
micusubcodeline --init

# 检查配置有效性
micusubcodeline --check

# 打印当前配置
micusubcodeline --print

# 进入 TUI 配置模式
micusubcodeline --config
```

### 主题覆盖

```bash
# 临时使用指定主题（覆盖配置文件设置）
micusubcodeline --theme cometix
micusubcodeline --theme minimal
micusubcodeline --theme gruvbox
micusubcodeline --theme nord
micusubcodeline --theme powerline-dark

# 或使用 ~/.claude/micusubcodeline/themes/ 目录下的自定义主题
micusubcodeline --theme my-custom-theme
```

### Claude Code 增强

```bash
# 禁用上下文警告并启用详细模式
micusubcodeline --patch /path/to/claude-code/cli.js
```

## 默认段落

显示：`目录 | Git 分支状态 | 模型 | 上下文窗口`

### Git 状态指示器

- 带 Nerd Font 图标的分支名
- 状态：`✓` 清洁，`●` 有更改，`⚠` 冲突
- 远程跟踪：`↑n` 领先，`↓n` 落后

### 模型显示

显示简化的 Claude 模型名称：
- `claude-3-5-sonnet` → `Sonnet 3.5`
- `claude-4-sonnet` → `Sonnet 4`

### 上下文窗口显示

基于转录文件分析的令牌使用百分比，包含上下文限制跟踪。

## 配置

MicuSubCodeLine 支持通过 TOML 文件和交互式 TUI 进行完整配置：

- **配置文件**: `~/.claude/micusubcodeline/config.toml`
- **交互式 TUI**: `micusubcodeline --config` 实时编辑配置并预览效果
- **主题文件**: `~/.claude/micusubcodeline/themes/*.toml` 自定义主题文件
- **自动初始化**: `micusubcodeline --init` 创建默认配置

### 可用段落

所有段落都支持配置：
- 启用/禁用切换
- 自定义分隔符和图标
- 颜色自定义
- 格式选项

支持的段落：目录、Git、模型、使用量、时间、成本、输出样式、订阅

## 系统要求

- **Git**: 版本 1.5+ (推荐 Git 2.22+ 以获得更好的分支检测)
- **终端**: 必须支持 Nerd Font 图标正常显示
  - 安装 [Nerd Font](https://www.nerdfonts.com/) 字体
  - 中文用户推荐: [Maple Font](https://github.com/subframe7536/maple-font) (支持中文的 Nerd Font)
  - 在终端中配置使用该字体
- **Claude Code**: 用于状态栏集成

## 开发

```bash
# 构建开发版本
cargo build

# 运行测试
cargo test

# 构建优化版本
cargo build --release
```

## 贡献

欢迎贡献！请随时提交 issue 或 pull request。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
