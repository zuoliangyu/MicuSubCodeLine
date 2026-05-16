//! Locked statusline renderer.
//!
//! This module produces a FIXED, non-configurable 3-line powerline statusline.
//! It deliberately bypasses the config / theme / segment system entirely so the
//! output cannot be altered by any user config, theme file or TUI edit.
//!
//! Layout (powerline rounded blocks, screenshot-matched palette):
//!
//!   行1  模型：<model>  上下文：<ctx>  ⌥ <branch>  (+N,−N)  合计：<X> t/s
//!   行2  会话：<dur>  费用：$<cost>  cwd: <path>
//!   行3  <plan>  每日：$d/$D  每周：$w/$W  到期：<N>天
//!
//! Glyphs follow the reference screenshot: fullwidth colon `：`, git mark `⌥`
//! (U+2325) and Unicode minus `−` (U+2212).

use crate::config::types::TranscriptEntry;
use crate::config::{InputData, ModelConfig};
use crate::core::segments::context_window::context_used_tokens;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// RGB background palette, eyeball-matched to the reference screenshot.
const INDIGO: (u8, u8, u8) = (76, 86, 160);
const GRAY: (u8, u8, u8) = (68, 71, 90);
const GREEN: (u8, u8, u8) = (95, 155, 75);
const ORANGE: (u8, u8, u8) = (200, 140, 55);
const PINK: (u8, u8, u8) = (190, 95, 145);
/// Foreground text color (near-white, bold) used on every block.
const TEXT: (u8, u8, u8) = (236, 236, 236);

/// Powerline rounded glyphs.
const CAP_LEFT: char = '\u{e0b6}'; // rounded left (start cap)
const CAP_RIGHT: char = '\u{e0b4}'; // rounded right (separator / end cap)

struct Block {
    text: String,
    bg: (u8, u8, u8),
}

impl Block {
    fn new(text: impl Into<String>, bg: (u8, u8, u8)) -> Self {
        Self {
            text: text.into(),
            bg,
        }
    }
}

/// Render the locked statusline. Always returns up to 3 lines.
pub fn render_locked(input: &InputData) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    // ---- Line 1: model / context / git / total speed -----------------------
    let mut l1 = vec![
        Block::new(format!("模型：{}", model_name(input)), INDIGO),
        Block::new(format!("上下文：{}", context_str(input)), GRAY),
    ];
    if let Some(branch) = git_branch(&input.workspace.current_dir) {
        l1.push(Block::new(format!("⌥ {}", branch), GREEN));
        let (ins, del) = git_changes(&input.workspace.current_dir);
        l1.push(Block::new(format!("(+{},−{})", ins, del), GRAY));
    }
    l1.push(Block::new(format!("合计：{}", total_speed(input)), ORANGE));
    lines.push(render_line(&l1));

    // ---- Line 2: session / cost / cwd --------------------------------------
    let l2 = vec![
        Block::new(format!("会话：{}", session_duration(input)), INDIGO),
        Block::new(format!("费用：{}", session_cost(input)), GRAY),
        Block::new(format!("cwd: {}", input.workspace.current_dir), GREEN),
    ];
    lines.push(render_line(&l2));

    // ---- Line 3: subscription (plan / daily / weekly / expiry) --------------
    if let Some(sub) = input.subscription.as_ref() {
        let mut l3 = vec![Block::new(sub.group_name.clone(), INDIGO)];
        l3.push(Block::new(
            format!(
                "每日：{}/{}",
                usd(sub.daily_used_usd),
                usd(sub.daily_limit_usd)
            ),
            GRAY,
        ));
        l3.push(Block::new(
            format!(
                "每周：{}/{}",
                usd(sub.weekly_used_usd),
                usd(sub.weekly_limit_usd)
            ),
            GREEN,
        ));
        if let Some(days) = days_until(sub.expires_at.as_deref()) {
            l3.push(Block::new(format!("到期：{}天", days), PINK));
        }
        lines.push(render_line(&l3));
    }

    lines
}

/// Render one line of powerline blocks with rounded caps and color transitions.
fn render_line(blocks: &[Block]) -> String {
    if blocks.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let (tr, tg, tb) = TEXT;

    // Start cap: rounded left edge in the first block's background color.
    let (fr, fg, fb) = blocks[0].bg;
    out.push_str(&format!("\x1b[38;2;{};{};{}m{}", fr, fg, fb, CAP_LEFT));

    for (i, b) in blocks.iter().enumerate() {
        let (br, bg, bb) = b.bg;
        out.push_str(&format!(
            "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m\x1b[1m {} \x1b[22m",
            br, bg, bb, tr, tg, tb, b.text
        ));
        // Separator into the next block, or rounded end cap after the last.
        match blocks.get(i + 1) {
            Some(next) => {
                let (nr, ng, nb) = next.bg;
                out.push_str(&format!(
                    "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{}",
                    nr, ng, nb, br, bg, bb, CAP_RIGHT
                ));
            }
            None => {
                out.push_str(&format!(
                    "\x1b[49m\x1b[38;2;{};{};{}m{}\x1b[0m",
                    br, bg, bb, CAP_RIGHT
                ));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Field computations
// ---------------------------------------------------------------------------

fn model_name(input: &InputData) -> String {
    let cfg = ModelConfig::load();
    let raw = cfg
        .get_display_name(&input.model.id)
        .unwrap_or_else(|| input.model.display_name.clone());
    // Strip a trailing parenthetical like " (1M)" the way ccstatusline does.
    match raw.find(" (") {
        Some(idx) if raw.ends_with(')') => raw[..idx].to_string(),
        _ => raw,
    }
}

fn context_str(input: &InputData) -> String {
    match context_used_tokens(&input.transcript_path) {
        Some(t) => format_tokens(t),
        None => "-".to_string(),
    }
}

fn format_tokens(n: u32) -> String {
    let n = n as f64;
    if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1000.0 {
        format!("{:.1}k", n / 1000.0)
    } else {
        format!("{}", n as u32)
    }
}

fn git_branch(dir: &str) -> Option<String> {
    let out = Command::new("git")
        .args(["--no-optional-locks", "branch", "--show-current"])
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let b = String::from_utf8(out.stdout).ok()?.trim().to_string();
    if b.is_empty() {
        // Could be a detached HEAD inside a real repo; treat as no branch line.
        None
    } else {
        Some(b)
    }
}

fn git_changes(dir: &str) -> (u32, u32) {
    let mut ins = 0u32;
    let mut del = 0u32;
    for args in [
        ["--no-optional-locks", "diff", "--shortstat"].as_slice(),
        ["--no-optional-locks", "diff", "--cached", "--shortstat"].as_slice(),
    ] {
        if let Ok(out) = Command::new("git").args(args).current_dir(dir).output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                ins += parse_shortstat(&s, "insertion");
                del += parse_shortstat(&s, "deletion");
            }
        }
    }
    (ins, del)
}

/// Extract the count preceding `keyword` (e.g. "insertion"/"deletion") from a
/// `git diff --shortstat` line: " 3 files changed, 37 insertions(+), 1 deletion(-)".
fn parse_shortstat(s: &str, keyword: &str) -> u32 {
    for part in s.split(',') {
        if part.contains(keyword) {
            if let Some(num) = part.split_whitespace().next() {
                return num.parse().unwrap_or(0);
            }
        }
    }
    0
}

fn total_speed(input: &InputData) -> String {
    let tokens = transcript_total_tokens(&input.transcript_path);
    let duration_ms = input
        .cost
        .as_ref()
        .and_then(|c| c.total_api_duration_ms.or(c.total_duration_ms))
        .unwrap_or(0);
    if tokens == 0 || duration_ms == 0 {
        return "—".to_string();
    }
    let tps = tokens as f64 / (duration_ms as f64 / 1000.0);
    if tps >= 1000.0 {
        format!("{:.1}k t/s", tps / 1000.0)
    } else {
        format!("{:.1} t/s", tps)
    }
}

/// Sum input + output tokens over every assistant message in the transcript.
fn transcript_total_tokens(transcript_path: &str) -> u64 {
    let file = match fs::File::open(transcript_path) {
        Ok(f) => f,
        Err(_) => return 0,
    };
    let mut total: u64 = 0;
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
            if entry.r#type.as_deref() == Some("assistant") {
                if let Some(usage) = entry.message.and_then(|m| m.usage) {
                    let n = usage.normalize();
                    total += n.input_tokens as u64 + n.output_tokens as u64;
                }
            }
        }
    }
    total
}

fn session_duration(input: &InputData) -> String {
    let ms = input
        .cost
        .as_ref()
        .and_then(|c| c.total_duration_ms)
        .unwrap_or(0);
    let total_min = ms / 60_000;
    if total_min < 1 {
        return "<1分".to_string();
    }
    let h = total_min / 60;
    let m = total_min % 60;
    if h == 0 {
        format!("{}分", m)
    } else if m == 0 {
        format!("{}时", h)
    } else {
        format!("{}时 {}分", h, m)
    }
}

fn session_cost(input: &InputData) -> String {
    match input.cost.as_ref().and_then(|c| c.total_cost_usd) {
        Some(c) => usd(c),
        None => "$0".to_string(),
    }
}

fn usd(amount: f64) -> String {
    if amount < 0.005 {
        "$0".to_string()
    } else {
        format!("${:.2}", amount)
    }
}

/// Whole days from now until the RFC3339 `expires_at` timestamp.
fn days_until(expires_at: Option<&str>) -> Option<i64> {
    let s = expires_at?;
    // Parse the leading "YYYY-MM-DD" date part only — enough for a day count.
    let date = s.get(..10)?;
    let mut it = date.split('-');
    let y: i64 = it.next()?.parse().ok()?;
    let m: i64 = it.next()?.parse().ok()?;
    let d: i64 = it.next()?.parse().ok()?;
    let expiry_days = days_from_civil(y, m, d);

    let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let now_days = now_secs / 86_400;

    Some((expiry_days - now_days).max(0))
}

/// Days since Unix epoch for a civil (proleptic Gregorian) date.
/// Howard Hinnant's `days_from_civil` algorithm.
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
