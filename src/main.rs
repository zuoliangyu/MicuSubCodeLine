use micusubcodeline::cli::Cli;
use micusubcodeline::config::{Config, InputData};
use std::io::{self, IsTerminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_args();

    // Handle configuration commands
    if cli.init {
        use micusubcodeline::config::InitResult;
        match Config::init()? {
            InitResult::Created(path) => println!("Created config at {}", path.display()),
            InitResult::AlreadyExists(path) => {
                println!("Config already exists at {}", path.display())
            }
        }
        return Ok(());
    }

    if cli.print {
        let mut config = Config::load().unwrap_or_else(|_| Config::default());

        // Apply theme override if provided
        if let Some(theme) = cli.theme {
            config = micusubcodeline::ui::themes::ThemePresets::get_theme(&theme);
        }

        config.print()?;
        return Ok(());
    }

    if cli.check {
        let config = Config::load()?;
        config.check()?;
        println!("✓ Configuration valid");
        return Ok(());
    }

    if cli.init_subscription {
        use micusubcodeline::utils::SubscriptionApi;

        println!("🔍 检测 API Key 状态...\n");
        if let Some(api) = SubscriptionApi::load() {
            println!("✅ API Key 已检测到");
            if let Some(sub) = api.get_subscription_info() {
                println!("   分组: {}", sub.group_name);
                println!("   今日消费: ${:.4}", sub.daily_used_usd);
            } else {
                println!("   ⚠️  Key 已读取但无法获取订阅信息，请检查 Key 是否有效");
            }
        } else {
            println!("❌ 未检测到 API Key");
            println!("\n   支持的读取位置（按优先级）：");
            println!("   1. ~/.claude/settings.local.json → env.ANTHROPIC_API_KEY / ANTHROPIC_AUTH_TOKEN");
            println!(
                "   2. ~/.claude/settings.json → env.ANTHROPIC_API_KEY / ANTHROPIC_AUTH_TOKEN"
            );
            println!("   3. 环境变量 ANTHROPIC_API_KEY / ANTHROPIC_AUTH_TOKEN");
            println!("   4. ~/.claude/micusubcodeline/subscription_config.txt");
        }
        return Ok(());
    }

    if cli.config {
        #[cfg(feature = "tui")]
        {
            micusubcodeline::ui::run_configurator()?;
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("TUI feature is not enabled. Please install with --features tui");
            std::process::exit(1);
        }
        return Ok(());
    }

    if cli.update {
        #[cfg(feature = "self-update")]
        {
            println!("Update feature not implemented in new architecture yet");
        }
        #[cfg(not(feature = "self-update"))]
        {
            println!("Update check not available (self-update feature disabled)");
        }
        return Ok(());
    }

    // Handle Claude Code patcher
    if let Some(claude_path) = cli.patch {
        use micusubcodeline::utils::ClaudeCodePatcher;

        println!("🔧 Claude Code Context Warning Disabler");
        println!("Target file: {}", claude_path);

        // Create backup in same directory
        let backup_path = format!("{}.backup", claude_path);
        std::fs::copy(&claude_path, &backup_path)?;
        println!("📦 Created backup: {}", backup_path);

        // Load and patch
        let mut patcher = ClaudeCodePatcher::new(&claude_path)?;

        println!("\n🔄 Applying patches...");
        let results = patcher.apply_all_patches();
        patcher.save()?;

        ClaudeCodePatcher::print_summary(&results);
        println!("💡 To restore warnings, replace your cli.js with the backup file:");
        println!("   cp {} {}", backup_path, claude_path);

        return Ok(());
    }

    // NOTE: the rendered statusline is HARD-LOCKED. It deliberately does not
    // read any user config, theme file, TUI edit or `--theme` override — the
    // layout below is fixed and cannot be modified by the user.

    // Check if stdin has data
    if io::stdin().is_terminal() {
        // No input data available, run setup wizard first
        use micusubcodeline::utils::SetupWizard;

        // Run setup wizard (it will check if configuration is needed)
        if let Err(e) = SetupWizard::run() {
            eprintln!("❌ 配置向导出错: {}", e);
        }

        // After setup wizard, show main menu if TUI is enabled
        #[cfg(feature = "tui")]
        {
            use micusubcodeline::ui::{MainMenu, MenuResult};

            if let Some(result) = MainMenu::run()? {
                match result {
                    MenuResult::LaunchConfigurator => {
                        micusubcodeline::ui::run_configurator()?;
                    }
                    MenuResult::InitConfig | MenuResult::CheckConfig => {
                        // These are now handled internally by the menu
                        // and should not be returned, but handle gracefully
                    }
                    MenuResult::Exit => {
                        // Exit gracefully
                    }
                }
            }
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("No input data provided and TUI feature is not enabled.");
            eprintln!("Usage: echo '{{...}}' | micusubcodeline");
            eprintln!("   or: micusubcodeline --help");
        }
        return Ok(());
    }

    // Read Claude Code data from stdin
    let stdin = io::stdin();
    let mut input: InputData = serde_json::from_reader(stdin.lock())?;

    // Try to fetch subscription data from API
    if let Some(api) = micusubcodeline::utils::SubscriptionApi::load() {
        if let Some(subscription) = api.get_subscription_info() {
            input.subscription = Some(subscription);
        }
    }

    // Render the fixed, non-configurable statusline.
    for line in micusubcodeline::core::render_locked(&input) {
        println!("{}", line);
    }

    Ok(())
}
