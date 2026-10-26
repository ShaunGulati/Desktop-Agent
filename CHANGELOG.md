# Changelog

All notable changes to this project are documented here, following
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Floating, transparent pet window that stays visible over full-screen apps.
- `⌥ Space` global hotkey and a menu-bar item to show or hide the pet.
- Sprite-sheet skin format (`skin.json` plus one PNG strip per state) with a placeholder skin.
- CapriBridge Swift package linked into the Rust core over a C ABI.
- Hotkey-to-first-paint latency logging against the 100 ms M0 budget.
- `pnpm verify` and a GitHub Actions workflow running web, Swift and Rust checks.
