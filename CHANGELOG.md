# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of `ipinfo` CLI tool
- Add `.agents/skills/ipinfo/SKILL.md` following the [Agent Skills](https://agentskills.io/) open standard, with YAML frontmatter (`license`, `compatibility`, `metadata`, `allowed-tools`) and structured body content for progressive disclosure
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
