# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-05-21

### Added

- **model**: Recognize Claude 4.7 series (`opus-4-7`, `sonnet-4-7`, `haiku-4-7`,
  including the 1M-context `[1m]` variants) so the statusline shows e.g.
  `Opus 4.7 1M` instead of falling through to the generic `Claude 1M` fallback.
- **subscription**: Detect "wallet balance" mode (non-subscription users) from
  the `/v1/usage` response and surface it as a dedicated `balance` field on
  the parsed `Subscription`.

### Fixed

- **statusline**: Line 3 now shows `今日:$X | 余额:$Y` for non-subscription
  users instead of the meaningless `每日:$X/$0 | 每周:$0/$balance`. The previous
  code overloaded `weekly_limit_usd` with the wallet balance, which the renderer
  then displayed as a weekly limit.

## [2.0.0] - 2026-05-16

### Added

- **statusline**: New hard-locked, non-configurable 3-line powerline renderer
  (`src/core/locked.rs`). The output is fixed and cannot be changed by any
  config file, theme file, `--theme` flag or TUI edit.
  - Line 1: model · context tokens · git branch · git changes `(+N,−N)` · total throughput (t/s)
  - Line 2: session duration · session cost · working directory
  - Line 3: plan name · daily $used/$limit · weekly $used/$limit · days to expiry
- **subscription**: Parse `daily_limit_usd` and `expires_at` from the
  `/v1/usage` response; surface daily limit and plan expiry on the statusline.

### Changed

- **subscription**: Switch the subscription site to `https://sub.micuapi.ai`
  (API paths unchanged). [**BREAKING**]
- **config**: The rendered statusline no longer reads `config.toml`, theme
  files, the `--theme` override or TUI edits. Those commands still exist for
  compatibility but no longer affect the displayed statusline. [**BREAKING**]
- **docs**: Chinese README is now the default (`README.md`); English moved to
  `README.en.md`. Showcase image replaced with `assets/展示图.png`.

### Fixed

- **clippy**: Resolve `collapsible_match` lint in the main menu key handling so
  `cargo clippy -- -D warnings` passes on current stable.

[2.1.0]: https://github.com/zuoliangyu/MicuSubCodeLine/releases/tag/v2.1.0
[2.0.0]: https://github.com/zuoliangyu/MicuSubCodeLine/releases/tag/v2.0.0
