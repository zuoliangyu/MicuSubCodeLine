# MicuSubCodeLine - Sub2API 订阅信息扩展版

基于 [CCometixLine](https://github.com/Haleclipse/CCometixLine) 二次开发，添加了 Sub2API 订阅信息显示功能。

## 新功能

### 订阅信息显示

在 Claude Code 状态栏中实时显示您的 Sub2API 订阅信息：
- 订阅分组名称（如 MICU-Ultra）
- 今日/本周费用统计
- 每周限额显示
- 额度刷新时间倒计时

**显示示例：**
```
💰 MICU-Ultra | 今日:$2.48 本周:$68.80/$140.00 | 刷新:9小时32分
```

## 快速开始

### 1. 编译项目

```bash
cd MicuSubCodeLine
cargo build --release
```

编译完成后，可执行文件位于：
- **Windows**: `target\release\micusubcodeline.exe`
- **Linux/macOS**: `target/release/micusubcodeline`

### 2. 自动安装（推荐）

**只需双击运行即可！**

程序会自动执行以下操作：
1. 检测是否已安装到 `~/.claude/micusubcodeline/` 目录
2. 如果未安装，自动创建目录并复制可执行文件
3. 从安装目录重新启动程序
4. 启动配置向导，引导您配置 API Key
5. 自动初始化默认配置文件

**安装位置：**
- **Windows**: `C:\Users\你的用户名\.claude\micusubcodeline\`
- **Linux/macOS**: `~/.claude/micusubcodeline/`

### 3. 初始化订阅配置

**方式一：双击运行（推荐）**

双击可执行文件，程序会自动：
- 安装到标准目录
- 检测配置并启动配置向导
- 引导您配置 API Key
- 自动初始化配置文件

**方式二：使用命令行**

```bash
# 创建订阅配置文件
micusubcodeline --init-subscription
```

这会在 `~/.claude/micusubcodeline/` 目录创建 `subscription_config.txt` 文件。

### 4. 配置 API Key

配置文件位置：
- **Windows**: `C:\Users\你的用户名\.claude\micusubcodeline\subscription_config.txt`
- **Linux/macOS**: `~/.claude/micusubcodeline/subscription_config.txt`

#### 获取 API Key 方法：

1. 打开浏览器，访问 https://sub.micuapi.ai 并登录
2. 进入 **API Keys** 管理页面
3. 创建或复制您的 API Key（格式: `sk-xxx`）
4. 将复制的值粘贴到 `subscription_config.txt` 文件中

**配置文件格式：**
```
# Sub2API 订阅配置
# 请在下方填写您的 API Key（从 Sub2API 面板获取，格式: sk-xxx）
# 配置文件位置: ~/.claude/micusubcodeline/subscription_config.txt

sk-xxxxxxxxxxxxxxxxxxxxxxxx
```

### 5. 配置 Claude Code

编辑 Claude Code 的 `settings.json`：

**Windows:**
```json
{
  "statusLine": {
    "type": "command",
    "command": "C:\\Users\\你的用户名\\.claude\\micusubcodeline\\micusubcodeline.exe",
    "padding": 0
  }
}
```

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

### 6. 重启 Claude Code

保存设置后，重启 Claude Code 即可看到订阅信息！

## 功能说明

### 订阅信息 Segment

默认情况下，订阅信息 segment 是**启用**的。如果您想关闭它：

```bash
# 进入 TUI 配置界面
micusubcodeline --config
```

在界面中找到 "Subscription" segment，按 `Enter` 切换启用/禁用状态。

### 显示内容

订阅信息包含以下字段：
- **分组名称**: 您的订阅分组（如 MICU-Ultra）
- **今日费用**: 当天已使用的费用
- **本周费用/限额**: 本周已用/本周总限额
- **刷新时间**: 距离下次额度刷新的时间

### 自动刷新

- 订阅数据会在每次 Claude Code 刷新状态栏时更新
- API 调用超时时间为 5 秒
- 如果网络异常，segment 将不显示

## 自定义配置

### 修改订阅 segment 样式

使用 TUI 配置界面：

```bash
micusubcodeline --config
```

可以自定义：
- 图标（Plain 模式 💰 / Nerd Font 模式）
- 颜色（图标颜色、文本颜色）
- 启用/禁用

### 修改 segment 顺序

在 TUI 界面中，使用方向键调整 segment 顺序，订阅信息默认在最后。

### 主题配置

订阅 segment 支持所有内置主题：
```bash
micusubcodeline --theme cometix
micusubcodeline --theme minimal
micusubcodeline --theme gruvbox
micusubcodeline --theme nord
```

## 故障排查

### 问题1: 订阅信息不显示

**可能原因：**
1. 未配置 `subscription_config.txt`
2. API Key 无效
3. 网络连接问题
4. subscription segment 被禁用

**解决方法：**
```bash
# 1. 检查配置文件是否存在
micusubcodeline --init-subscription

# 2. 验证 API Key 是否正确（检查文件内容）
# Windows: notepad %USERPROFILE%\.claude\micusubcodeline\subscription_config.txt
# Linux/macOS: cat ~/.claude/micusubcodeline/subscription_config.txt

# 3. 测试网络连接
# 访问 https://sub.micuapi.ai/api/v1/subscriptions/summary

# 4. 检查 segment 是否启用
micusubcodeline --config
```

### 问题2: API Key 无效

如果 API Key 不可用，请重新获取：
1. 登录 https://sub.micuapi.ai
2. 进入 API Keys 管理页面，创建新的 API Key
3. 更新配置文件：
   - Windows: `C:\Users\你的用户名\.claude\micusubcodeline\subscription_config.txt`
   - Linux/macOS: `~/.claude/micusubcodeline/subscription_config.txt`

### 问题3: 编译失败

确保已安装 Rust 工具链：
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或 Windows 上使用
# https://rustup.rs/
```

## API 说明

### 使用的 API 端点

本扩展调用两个 Sub2API 接口：

1. **订阅摘要 API**
   - 端点: `GET /api/v1/subscriptions/summary`
   - 获取: 分组名称、费用统计

2. **订阅进度 API**
   - 端点: `GET /api/v1/subscriptions/progress`
   - 获取: 刷新时间、剩余天数

### 数据更新频率

- 每次 Claude Code 刷新状态栏时更新
- 无缓存机制（每次都是实时数据）
- API 调用超时：5 秒

## 安全说明

- `subscription_config.txt` 包含敏感的 API Key
- 配置文件位置：`~/.claude/micusubcodeline/subscription_config.txt`
- 请勿将该文件提交到公开仓库
- API Key 具有账号权限，请妥善保管
- 分发程序时不要包含配置文件，让用户自行配置
- 程序首次运行会自动安装到 `~/.claude/micusubcodeline/` 目录

## 打包发布

### 编译 Release 版本

```bash
cargo build --release

# Windows: target\release\micusubcodeline.exe (约 8-15MB)
# Linux: target/release/micusubcodeline (约 6-10MB)
```

### 分发

```bash
# Windows
7z a micusubcodeline-windows.zip target\release\micusubcodeline.exe README_SUB2API.md

# Linux
tar czf micusubcodeline-linux.tar.gz target/release/micusubcodeline README_SUB2API.md
```

**注意：**
- 分发时不要包含 `subscription_config.txt` 文件
- 用户首次运行时会自动启动配置向导

## 鸣谢

本项目基于 [CCometixLine](https://github.com/Haleclipse/CCometixLine) 进行二次开发。

## 许可证

本扩展版本继承原项目的 MIT 许可证。
