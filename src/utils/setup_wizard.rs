use std::path::PathBuf;

pub struct SetupWizard;

impl SetupWizard {
    /// 运行首次配置向导
    pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
        // 首先检查并执行自动安装
        Self::auto_install()?;

        Self::print_welcome();

        // 检测 API Key 状态
        Self::check_api_key_status();

        // 自动执行 init 初始化
        println!("\n🔧 正在自动初始化配置...");
        match crate::config::Config::init() {
            Ok(crate::config::InitResult::Created(path)) => {
                println!("✅ 配置文件已创建: {}", path.display());
            }
            Ok(crate::config::InitResult::AlreadyExists(path)) => {
                println!("✅ 配置文件已存在: {}", path.display());
            }
            Err(e) => {
                eprintln!("⚠️  配置初始化失败: {}", e);
                eprintln!("   您可以稍后手动运行: micusubcodeline --init");
            }
        }

        // 显示下一步指引
        Self::print_next_steps();

        Ok(true)
    }

    fn print_welcome() {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                    MicuSubCodeLine 配置向导                     ║");
        println!("║            Claude Code StatusLine with Sub2API              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
    }

    /// 检测 API Key 来源并展示状态
    fn check_api_key_status() {
        use crate::utils::SubscriptionApi;

        println!("\n🔍 正在检测 API Key...");

        if SubscriptionApi::load().is_some() {
            println!(
                "✅ 已自动检测到 API Key（来源: {}）",
                Self::detect_key_source()
            );
        } else {
            println!("\n⚠️  未检测到 API Key");
            println!("   请确保在 Claude Code 的 settings.json 中配置了 API Key：");
            println!();
            println!("   {{");
            println!("     \"env\": {{");
            println!("       \"ANTHROPIC_AUTH_TOKEN\": \"sk-xxx\"");
            println!("     }}");
            println!("   }}");
            println!();
            println!("   支持的字段名：ANTHROPIC_API_KEY 或 ANTHROPIC_AUTH_TOKEN");
            println!("   支持的读取位置（按优先级）：");
            println!("   1. ~/.claude/settings.local.json");
            println!("   2. ~/.claude/settings.json");
            println!("   3. 环境变量");
            println!("   4. ~/.claude/micusubcodeline/subscription_config.txt");
        }
    }

    /// 检测 API Key 的实际来源
    fn detect_key_source() -> &'static str {
        // 按优先级依次检查
        if let Some(home) = dirs::home_dir() {
            let claude_dir = home.join(".claude");

            let local_settings = claude_dir.join("settings.local.json");
            if Self::has_api_key_in_settings(&local_settings) {
                return "settings.local.json";
            }

            let settings = claude_dir.join("settings.json");
            if Self::has_api_key_in_settings(&settings) {
                return "settings.json";
            }
        }

        for field in &["ANTHROPIC_API_KEY", "ANTHROPIC_AUTH_TOKEN"] {
            if std::env::var(field)
                .ok()
                .filter(|k| !k.trim().is_empty())
                .is_some()
            {
                return "环境变量";
            }
        }

        "subscription_config.txt"
    }

    fn has_api_key_in_settings(path: &PathBuf) -> bool {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };
        let settings: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let env = match settings.get("env") {
            Some(e) => e,
            None => return false,
        };
        ["ANTHROPIC_API_KEY", "ANTHROPIC_AUTH_TOKEN"]
            .iter()
            .any(|field| {
                env.get(*field)
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.trim().is_empty())
                    .is_some()
            })
    }

    /// 自动安装到 ~/.claude/micusubcodeline/ 目录
    fn auto_install() -> Result<(), Box<dyn std::error::Error>> {
        // 获取当前可执行文件路径
        let current_exe = std::env::current_exe()?;

        // 获取目标安装目录
        let install_dir = dirs::home_dir()
            .ok_or("无法确定用户主目录")?
            .join(".claude")
            .join("micusubcodeline");

        // 目标可执行文件路径
        let target_exe = install_dir.join(current_exe.file_name().ok_or("无法获取可执行文件名")?);

        // 检查是否已经在安装目录中运行
        if current_exe.canonicalize().ok() == target_exe.canonicalize().ok() {
            // 已经在目标目录中，无需安装
            return Ok(());
        }

        // 创建安装目录（如果不存在）
        if !install_dir.exists() {
            println!("📁 创建安装目录: {}", install_dir.display());
            std::fs::create_dir_all(&install_dir)?;
        }

        // 复制可执行文件到安装目录
        println!("📦 正在安装 MicuSubCodeLine...");
        println!("   从: {}", current_exe.display());
        println!("   到: {}", target_exe.display());

        std::fs::copy(&current_exe, &target_exe)?;

        #[cfg(unix)]
        {
            // 在Unix系统上设置可执行权限
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&target_exe)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&target_exe, perms)?;
        }

        println!("✅ 安装成功！");
        println!("\n💡 提示:");
        println!("   程序已安装到: {}", install_dir.display());
        println!("   配置文件将保存在同一目录下");
        println!("\n🔄 现在将从安装目录启动程序...\n");

        // 从安装目录重新启动程序
        let status = std::process::Command::new(&target_exe).spawn()?.wait()?;

        // 退出当前进程
        std::process::exit(status.code().unwrap_or(0));
    }

    fn print_next_steps() {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                        下一步操作                            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        println!("📋 配置 Claude Code 的 settings.json，添加 statusLine：");
        println!();

        #[cfg(target_os = "windows")]
        {
            let exe_path = std::env::current_exe()
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| {
                    "C:\\Users\\你的用户名\\.claude\\micusubcodeline\\micusubcodeline.exe"
                        .to_string()
                });

            println!("   {{");
            println!("     \"statusLine\": {{");
            println!("       \"type\": \"command\",");
            println!(
                "       \"command\": \"{}\",",
                exe_path.replace("\\", "\\\\")
            );
            println!("       \"padding\": 0");
            println!("     }}");
            println!("   }}");
        }

        #[cfg(not(target_os = "windows"))]
        {
            let exe_path = std::env::current_exe()
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| "~/.claude/micusubcodeline/micusubcodeline".to_string());

            println!("   {{");
            println!("     \"statusLine\": {{");
            println!("       \"type\": \"command\",");
            println!("       \"command\": \"{}\",", exe_path);
            println!("       \"padding\": 0");
            println!("     }}");
            println!("   }}");
        }

        println!();
        println!("   重启 Claude Code 后即可在状态栏看到信息！");
        println!();
        println!("💡 提示:");
        println!("   - API Key 会自动从 Claude Code settings 中读取，无需额外配置");
        println!("   - 如需自定义样式，运行: micusubcodeline --config");
        println!();
    }
}
