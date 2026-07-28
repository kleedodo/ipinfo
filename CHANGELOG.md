# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Fix CI workflow for cross-platform builds: update deprecated Node.js 20 actions (v4→v5), add aarch64 Linux cross-compilation linker, remove unbuildable OpenBSD target, add FreeBSD cross-compilation via cargo-zigbuild
- Update CI actions to latest versions: `actions/checkout` v5→v7, `actions/upload-artifact` v5→v7, `actions/download-artifact` v5→v8, `softprops/action-gh-release` v2→v3
- Fix release job to also run on `workflow_dispatch` so manually triggered builds add assets to a release
- Add `tag` input to `workflow_dispatch` so users can specify which git tag/release to attach build artifacts to

### Fixed

- Fix `softprops/action-gh-release@v3` input parameter name from `tag` to `tag_name` — the action no longer accepts `tag` and requires `tag_name` to specify the release tag
- Add `shell: bash` to the "Build release" step to prevent PowerShell from misinterpreting bash `if` syntax on Windows runners
- Use `aarch64-linux-gnu-strip` for aarch64 cross-compiled binaries instead of the host `strip`, which cannot handle foreign binary formats

## [0.1.0] - 2026-07-28

### Added

- Initial release of `ipinfo` CLI tool
- Add `.agents/skills/ipinfo/SKILL.md` following the [Agent Skills](https://agentskills.io/) open standard, with YAML frontmatter (`license`, `compatibility`, `metadata`, `allowed-tools`) and structured body content for progressive disclosure
  - Add install section documenting `cargo install --path .` to install binary to `~/.cargo/bin`
- IP geolocation lookup via dklyIPdatabase API
- Support for optional IP address argument (defaults to requester's public IP)
- API key authentication via `config.json`
- Configuration via `config.json` files (reads from `./config.json` or `~/.config/ipinfo/config.json`)
  - `providers.dkly.key` field in config JSON for API authentication
- Markdown-formatted output with the following sections:
  - Basic Info (IP, type, hostname)
  - Connection (ASN, organization, type)
  - Location (continent, country, region, city, postal, coordinates)
  - Time Zone (ID, abbreviation, UTC offset)
  - Currency (code, name, symbol)
  - Security (VPN, proxy, Tor, threat flags)
- Upgrade `reqwest` from `0.12` to `0.13.4`
- Add GitHub Actions CI workflow (`.github/workflows/build.yml`) to cross-compile `ipinfo` for multiple platforms (Linux x86_64/arm64, macOS arm64, Windows x86_64, OpenBSD x86_64) and upload build artifacts to GitHub
