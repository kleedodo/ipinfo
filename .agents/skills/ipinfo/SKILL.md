---
name: ipinfo
description: Use when looking up IP geolocation, network details, ASN info, or security indicators for any IP address.
license: MIT
compatibility: Requires a bash-capable agent and network access to ipinfo.dkly.net.
metadata:
  author: kleedodo
  version: "0.1.0"
  repo: https://github.com/kleedodo/ipinfo
allowed-tools: bash
---

# ipinfo

Look up IP geolocation, network, and security information for any address.

## Quick Start

```bash
# Look up your own public IP
ipinfo

# Look up a specific IP address
ipinfo 8.8.8.8
```

## Install

Clone the repository and install from source:

```bash
git clone https://github.com/kleedodo/ipinfo.git
cd ipinfo
cargo install --path .
```

This builds the project and installs the `ipinfo` binary to `~/.cargo/bin`, which is typically already on your `PATH`.

## When to use

- The user wants to know the geolocation, owner, or network details of an IP address.
- The user wants to check security indicators (VPN, proxy, Tor, threat) for an IP.
- The user needs currency or time zone information associated with an IP's location.
- Scripting or automation that requires IP intelligence in a structured Markdown format.

## When NOT to use

- For bulk IP lookups at scale — the tool is designed for single-query interactive use.
- For non-IP-related network diagnostics (port scanning, traceroute, etc.).

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

## API

The tool queries the dkly IP database API at `https://ipinfo.dkly.net/api/`. It accepts the query parameters `key` (API key) and `ip` (IP address to look up; omitted for the requester's own IP). The API returns JSON which is parsed and rendered as Markdown.
