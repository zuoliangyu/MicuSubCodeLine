use crate::config::types::Subscription;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "subscription_config.txt";
const BASE_URL: &str = "https://sub.openclaudecode.cn";

/// GET /v1/usage 统一响应结构
#[derive(Deserialize)]
struct UsageResponse {
    #[serde(rename = "isValid")]
    is_valid: Option<bool>,
    #[serde(rename = "planName")]
    plan_name: Option<String>,
    remaining: Option<f64>,
    subscription: Option<UsageSubscription>,
    balance: Option<f64>,
    usage: Option<UsageStats>,
}

#[derive(Deserialize)]
struct UsageSubscription {
    daily_usage_usd: Option<f64>,
    daily_limit_usd: Option<f64>,
    weekly_usage_usd: Option<f64>,
    weekly_limit_usd: Option<f64>,
    expires_at: Option<String>,
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
    pub fn load() -> Option<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&config_path).ok()?;
        // 读取第一个非空非注释行作为 API Key
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

    /// 检查配置文件是否存在
    pub fn config_exists() -> bool {
        Self::get_config_path()
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// 创建配置文件模板
    pub fn create_config_template() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path().ok_or("Cannot determine home directory")?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let template = "# MicuSubCodeLine 订阅配置\n\
                       # 请在下方填写您的 API Key（从 Sub2API 面板获取，格式: sk-xxx）\n\
                       #\n\
                       # 配置文件位置: ~/.claude/micusubcodeline/subscription_config.txt\n\
                       \n\
                       your_api_key_here";

        fs::write(&config_path, template)?;
        Ok(config_path)
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
        else if let Some(balance) = data.balance {
            Some(Subscription {
                group_name,
                daily_used_usd: today_cost,
                weekly_used_usd: 0.0,
                weekly_limit_usd: balance,
                resets_in_seconds: None,
            })
        } else {
            None
        }
    }
}
