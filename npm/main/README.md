# @zuolan/micusubcodeline

MicuSubCodeLine 是基于 Rust 的高性能 Claude Code 状态栏工具，集成 Git 信息、使用量跟踪、Sub2API 订阅信息显示和交互式 TUI 配置。

## 安装

```bash
npm install -g @zuolan/micusubcodeline
```

安装后自动复制到：`~/.claude/micusubcodeline/micusubcodeline`

## Claude Code 配置

在 `~/.claude/settings.json` 中添加 `statusLine`：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micusubcodeline/micusubcodeline",
    "padding": 0
  }
}
```

Windows 示例：

```json
{
  "statusLine": {
    "type": "command",
    "command": "C:/Users/你的用户名/.claude/micusubcodeline/micusubcodeline.exe",
    "padding": 0
  }
}
```

配置好后，MicuSubCodeLine 会自动从 Claude Code 的 `settings.json` 中读取 API Key，通过 Sub2API 接口实时获取订阅余额，**无需额外配置**。

## 使用

```bash
micusubcodeline --help        # 查看帮助
micusubcodeline --init        # 初始化配置与主题
micusubcodeline --config      # 打开配置面板
micusubcodeline --theme nord  # 指定主题运行
```

## 状态栏显示内容

模型 | 目录 | Git 分支状态 | 上下文窗口 | Sub2API 订阅信息

## 链接

- GitHub：https://github.com/zuoliangyu/MicuSubCodeLine
- 原作者：https://github.com/Haleclipse/CCometixLine
