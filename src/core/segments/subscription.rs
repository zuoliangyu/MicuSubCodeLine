use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct SubscriptionSegment;

impl SubscriptionSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for SubscriptionSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let subscription = input.subscription.as_ref()?;

        // Primary display: 订阅名称 + 费用
        let primary = format!(
            "{} | 今日:{} 本周:{}/{}",
            subscription.group_name,
            format_usd(subscription.daily_used_usd),
            format_usd(subscription.weekly_used_usd),
            format_usd(subscription.weekly_limit_usd)
        );

        // Secondary display: 刷新时间
        let secondary = if let Some(ref reset) = subscription.resets_in_seconds {
            format!("刷新:{}", format_time_remaining(*reset))
        } else {
            String::new()
        };

        let mut metadata = HashMap::new();
        metadata.insert("group_name".to_string(), subscription.group_name.clone());
        metadata.insert(
            "daily_cost".to_string(),
            subscription.daily_used_usd.to_string(),
        );
        metadata.insert(
            "weekly_cost".to_string(),
            subscription.weekly_used_usd.to_string(),
        );
        metadata.insert(
            "weekly_limit".to_string(),
            subscription.weekly_limit_usd.to_string(),
        );

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Subscription
    }
}

fn format_usd(amount: f64) -> String {
    if amount < 0.01 {
        "$0".to_string()
    } else {
        format!("${:.2}", amount)
    }
}

fn format_time_remaining(seconds: i64) -> String {
    if seconds <= 0 {
        return "已到期".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours >= 24 {
        let days = hours / 24;
        let remaining_hours = hours % 24;
        if remaining_hours > 0 {
            format!("{}天{}小时", days, remaining_hours)
        } else {
            format!("{}天", days)
        }
    } else if hours > 0 {
        format!("{}小时{}分", hours, minutes)
    } else {
        format!("{}分钟", minutes)
    }
}
