use micusubcodeline::cli::Cli;
use micusubcodeline::config::{Config, InputData};
use micusubcodeline::core::{collect_all_segments, StatusLineGenerator};
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

        if SubscriptionApi::config_exists() {
            println!("✓ Subscription configuration already exists");
            if let Some(path) = dirs::home_dir().map(|h| h.join(".claude").join("micusubcodeline").join("subscription_config.txt")) {
                println!("  Location: {}", path.display());
            }
        } else {
            match SubscriptionApi::create_config_template() {
                Ok(path) => {
                    println!("✓ Created subscription configuration template");
                    println!("  Location: {}", path.display());
                    println!("\n📝 Next steps:");
                    println!("  1. Open the file and replace 'your_jwt_token_here' with your JWT token");
                    println!("  2. To get your JWT token:");
                    println!("     - Login to https://sub.openclaudecode.cn");
                    println!("     - Press F12 to open DevTools");
                    println!("     - Go to Network tab");
                    println!("     - Refresh the page and filter 'me?' request");
                    println!("     - Click the request and view Headers");
                    println!("     - Find 'Authorization' field and copy the value after 'Bearer'");
                }
                Err(e) => {
                    eprintln!("❌ Failed to create configuration: {}", e);
                    std::process::exit(1);
                }
            }
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

    // Load configuration
    let mut config = Config::load().unwrap_or_else(|_| Config::default());

    // Apply theme override if provided
    if let Some(theme) = cli.theme {
        config = micusubcodeline::ui::themes::ThemePresets::get_theme(&theme);
    }

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

    // Collect segment data
    let segments_data = collect_all_segments(&config, &input);

    // Render statusline
    let generator = StatusLineGenerator::new(config);
    let statusline = generator.generate(segments_data);

    println!("{}", statusline);

    Ok(())
}
