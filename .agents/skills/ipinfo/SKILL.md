---
name: ipinfo
description: Use when looking up IP geolocation, network details, ASN info, or security indicators for any IP address. Returns structured Markdown output with location, connection, time zone, currency, and security data.
license: MIT
compatibility: Requires a bash-capable agent and network access to ipinfo.dkly.net; build from source with Rust toolchain or use a pre-built binary.
metadata:
  author: kleedodo
  version: "0.1.0"
  repo: https://github.com/kleedodo/ipinfo
allowed-tools: bash
---

# ipinfo

A Rust CLI tool that queries the **dklyIPdatabase** API and outputs IP information as formatted Markdown.

## Quick Start

```bash
# Look up your own public IP
ipinfo

# Look up a specific IP address
ipinfo 8.8.8.8
```

## Install

From the project root directory, install the binary to `~/.cargo/bin`:

```bash
cargo install --path .
```

This builds the project and places the `ipinfo` executable in your Cargo bin directory, which is typically already on your `PATH`.

## Output Format

The tool prints Markdown to stdout with the following sections:

| Section | Contents |
|---------|----------|
| **Basic Info** | IP address, type, hostname |
| **Connection** | ASN, organization, connection type |
| **Location** | Continent, country (with flag emoji), region, city, postal code, coordinates |
| **Time Zone** | Zone ID, abbreviation, UTC offset in seconds |
| **Currency** | Currency code, name, symbol |
| **Security** | VPN, proxy, Tor, and threat boolean flags |

## Configuration

The tool reads API keys from `config.json`. It searches for the config file in this order:

1. `./config.json` (local, takes precedence)
2. `~/.config/ipinfo/config.json` (global)

Use `config.json.example` in the project root as a template. When no key is configured, requests are made without authentication.

## When to use

- The user wants to know the geolocation, owner, or network details of an IP address.
- The user wants to check security indicators (VPN, proxy, Tor, threat) for an IP.
- The user needs currency or time zone information associated with an IP's location.
- Scripting or automation that requires IP intelligence in a structured Markdown format.

## When NOT to use

- For bulk IP lookups at scale — the tool is designed for single-query interactive use.
- For non-IP-related network diagnostics (port scanning, traceroute, etc.).

## Building from source

```bash
cargo build --release
```

The binary is produced at `target/release/ipinfo`. To install from the repository:

```bash
git clone https://github.com/kleedodo/ipinfo.git
cd ipinfo
cargo build --release
```

## API

The tool queries the dkly IP database API at `https://ipinfo.dkly.net/api/`. It accepts the query parameters `key` (API key) and `ip` (IP address to look up; omitted for the requester's own IP). The API returns JSON which is parsed and rendered as Markdown.
