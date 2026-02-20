use crate::config::types::Subscription;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "subscription_config.txt";
const BASE_URL: &str = "https://sub.openclaudecode.cn";

/// Claude Code settings 文件读取优先级（从高到低）
const SETTINGS_FILES: &[&str] = &["settings.local.json", "settings.json"];

/// settings.json 中可能存放 API Key 的字段名（按优先级）
const API_KEY_FIELDS: &[&str] = &[
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_AUTH_TOKEN",
];

/// GET /v1/usage 统一响应结构
#[derive(Deserialize)]
struct UsageResponse {
    #[serde(rename = "isValid")]
    is_valid: Option<bool>,
    #[serde(rename = "planName")]
    plan_name: Option<String>,
    subscription: Option<UsageSubscription>,
    balance: Option<f64>,
    usage: Option<UsageStats>,
}

#[derive(Deserialize)]
struct UsageSubscription {
    daily_usage_usd: Option<f64>,
    weekly_usage_usd: Option<f64>,
    weekly_limit_usd: Option<f64>,
    resets_in_seconds: Option<i64>,
}

#[derive(Deserialize)]
struct UsageStats {
    today: Option<UsagePeriod>,
}

#[derive(Deserialize)]
struct UsagePeriod {
    cost_usd: Option<f64>,
}

pub struct SubscriptionApi {
    api_key: String,
}

impl SubscriptionApi {
    /// 加载订阅API配置
    /// 优先级：settings.local.json > settings.json > 环境变量 > subscription_config.txt
    pub fn load() -> Option<Self> {
        // 1. 尝试从 Claude Code settings 文件读取
        if let Some(api_key) = Self::read_key_from_claude_settings() {
            return Some(Self { api_key });
        }

        // 2. 尝试从环境变量读取
        if let Some(api_key) = Self::read_key_from_env() {
            return Some(Self { api_key });
        }

        // 3. 回退到 subscription_config.txt
        Self::read_key_from_config_file()
    }

    /// 从 Claude Code settings 文件中读取 ANTHROPIC_API_KEY
    fn read_key_from_claude_settings() -> Option<String> {
        let home = dirs::home_dir()?;
        let claude_dir = home.join(".claude");

        for filename in SETTINGS_FILES {
            let path = claude_dir.join(filename);
            if let Some(key) = Self::extract_api_key_from_settings(&path) {
                return Some(key);
            }
        }

        None
    }

    /// 从单个 settings JSON 文件中提取 API Key
    fn extract_api_key_from_settings(path: &PathBuf) -> Option<String> {
        let content = fs::read_to_string(path).ok()?;
        let settings: serde_json::Value = serde_json::from_str(&content).ok()?;

        let env = settings.get("env")?;

        for field in API_KEY_FIELDS {
            if let Some(key) = env
                .get(*field)
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
            {
                return Some(key);
            }
        }

        None
    }

    /// 从环境变量读取 API Key
    fn read_key_from_env() -> Option<String> {
        for field in API_KEY_FIELDS {
            if let Some(key) = std::env::var(field)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
            {
                return Some(key);
            }
        }
        None
    }

    /// 从 subscription_config.txt 读取（原有逻辑）
    fn read_key_from_config_file() -> Option<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&config_path).ok()?;
        let api_key = content
            .lines()
            .find(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })?
            .trim()
            .to_string();

        if api_key.is_empty() || api_key == "your_api_key_here" {
            return None;
        }

        Some(Self { api_key })
    }

    /// 获取配置文件路径（~/.claude/micusubcodeline/subscription_config.txt）
    fn get_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| {
            home.join(".claude")
                .join("micusubcodeline")
                .join(CONFIG_FILE)
        })
    }

    /// 获取订阅信息（通过 GET /v1/usage 统一端点）
    pub fn get_subscription_info(&self) -> Option<Subscription> {
        let url = format!("{}/v1/usage", BASE_URL);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .ok()?;

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .ok()?;

        let data: UsageResponse = response.json().ok()?;

        // 检查是否有效
        if data.is_valid != Some(true) {
            return None;
        }

        let group_name = data.plan_name.unwrap_or_default();
        let today_cost = data
            .usage
            .as_ref()
            .and_then(|u| u.today.as_ref())
            .and_then(|t| t.cost_usd)
            .unwrap_or(0.0);

        // 订阅模式
        if let Some(sub) = &data.subscription {
            Some(Subscription {
                group_name,
                daily_used_usd: sub.daily_usage_usd.unwrap_or(today_cost),
                weekly_used_usd: sub.weekly_usage_usd.unwrap_or(0.0),
                weekly_limit_usd: sub.weekly_limit_usd.unwrap_or(0.0),
                resets_in_seconds: sub.resets_in_seconds,
            })
        }
        // 余额模式
        else {
            data.balance.map(|balance| Subscription {
                group_name,
                daily_used_usd: today_cost,
                weekly_used_usd: 0.0,
                weekly_limit_usd: balance,
                resets_in_seconds: None,
            })
        }
    }
}
