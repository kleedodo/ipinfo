# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - First Release

### Added

- Initial release of `ipinfo` CLI tool
- IP geolocation lookup via dklyIPdatabase API
- Support for optional IP address argument (defaults to requester's public IP)
- API key authentication via `DKLY_API_KEY` environment variable
- `.env` file support for configuration
- Markdown-formatted output with the following sections:
  - Basic Info (IP, type, hostname)
  - Connection (ASN, organization, type)
  - Location (continent, country, region, city, postal, coordinates)
  - Time Zone (ID, abbreviation, UTC offset)
  - Currency (code, name, symbol)
  - Security (VPN, proxy, Tor, threat flags)
