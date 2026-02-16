use std::io::{self, Write};
use std::path::PathBuf;

pub struct SetupWizard;

impl SetupWizard {
    /// 运行首次配置向导
    pub fn run() -> Result<bool, Box<dyn std::error::Error>> {
        // 首先检查并执行自动安装
        Self::auto_install()?;

        Self::print_welcome();

        // 检查配置文件
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                // 配置文件存在，检查是否是默认值
                if Self::is_default_token(&config_path)? {
                    println!("\n⚠️  检测到配置文件，但 API Key 仍是默认值");
                    Self::prompt_token_setup(&config_path)?;
                } else {
                    println!("\n✅ 配置文件已存在且已配置");
                    println!("📍 配置文件位置: {}", config_path.display());

                    if Self::prompt_yes_no("\n是否要重新配置？(y/n): ")? {
                        Self::prompt_token_setup(&config_path)?;
                    }
                }
            } else {
                // 配置文件不存在，创建新的
                println!("\n📝 未检测到配置文件，开始首次配置...\n");
                Self::create_and_setup(&config_path)?;
            }

            // 显示下一步指引
            Self::print_next_steps(&config_path);

            // 配置完成后自动执行init初始化
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

            Ok(true)
        } else {
            eprintln!("❌ 错误: 无法确定配置文件路径");
            Ok(false)
        }
    }

    fn print_welcome() {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                    MicuSubCodeLine 配置向导                     ║");
        println!("║            Claude Code StatusLine with Sub2API              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
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
        let target_exe = install_dir.join(
            current_exe
                .file_name()
                .ok_or("无法获取可执行文件名")?
        );

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
        let status = std::process::Command::new(&target_exe)
            .spawn()?
            .wait()?;

        // 退出当前进程
        std::process::exit(status.code().unwrap_or(0));
    }

    fn get_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| {
            home.join(".claude").join("micusubcodeline").join("subscription_config.txt")
        })
    }

    fn is_default_token(path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let token = content
            .lines()
            .find(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .unwrap_or("")
            .trim();

        Ok(token.is_empty() || token == "your_api_key_here")
    }

    fn create_and_setup(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // 创建配置目录
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 创建配置文件模板
        let template = "# your_api_key_here";

        std::fs::write(&config_path, template)?;
        println!("✅ 配置文件已创建: {}", config_path.display());

        Self::prompt_token_setup(config_path)?;

        Ok(())
    }

    fn prompt_token_setup(config_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                    配置 API Key                              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        println!("📋 获取 API Key:");
        println!("   1. 登录 Sub2API 面板: https://sub.openclaudecode.cn");
        println!("   2. 进入 API Keys 管理页面");
        println!("   3. 创建或复制您的 API Key（格式: sk-xxx）");
        println!();

        print!("🔑 请输入您的 API Key: ");
        io::stdout().flush()?;

        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key)?;
        let api_key = api_key.trim().to_string();

        if api_key.is_empty() {
            println!("\n⚠️  未输入 API Key，您可以稍后手动编辑配置文件:");
            println!("   {}", config_path.display());
        } else {
            // 写入配置文件
            let content = format!(
                "# MicuSubCodeLine 订阅配置\n\
                 # API Key (从 Sub2API 面板获取)\n\
                 \n\
                 {}",
                api_key
            );
            std::fs::write(config_path, content)?;
            println!("\n✅ API Key 已保存到: {}", config_path.display());
        }

        Ok(())
    }

    fn print_next_steps(config_path: &PathBuf) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                        下一步操作                            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        println!("📋 步骤 1: 确认配置文件");
        println!("   位置: {}", config_path.display());
        println!("   确保已正确填写 API Key");
        println!();
        println!("📋 步骤 2: 配置Claude Code");
        println!("   编辑Claude Code的 settings.json 文件");
        println!();

        #[cfg(target_os = "windows")]
        {
            let exe_path = std::env::current_exe()
                .ok()
                .and_then(|p| p.to_str().map(String::from))
                .unwrap_or_else(|| "C:\\Users\\你的用户名\\.claude\\micusubcodeline\\micusubcodeline.exe".to_string());

            println!("   添加以下配置:");
            println!("   {{");
            println!("     \"statusLine\": {{");
            println!("       \"type\": \"command\",");
            println!("       \"command\": \"{}\",", exe_path.replace("\\", "\\\\"));
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

            println!("   添加以下配置:");
            println!("   {{");
            println!("     \"statusLine\": {{");
            println!("       \"type\": \"command\",");
            println!("       \"command\": \"{}\",", exe_path);
            println!("       \"padding\": 0");
            println!("     }}");
            println!("   }}");
        }

        println!();
        println!("📋 步骤 3: 重启Claude Code");
        println!("   重启后即可在状态栏看到订阅信息！");
        println!();
        println!("💡 提示:");
        println!("   - 订阅信息格式: 分组名 | 今日费用 本周费用/限额 | 刷新时间");
        println!("   - 如需自定义样式，运行: micusubcodeline --config");
        println!("   - 如需重新配置 API Key，运行: micusubcodeline --init-subscription");
        println!();
    }

    fn prompt_yes_no(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let answer = input.trim().to_lowercase();
        Ok(answer == "y" || answer == "yes" || answer == "是")
    }
}
