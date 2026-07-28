# ipinfo

A Rust CLI tool that queries IP information from the **dklyIPdatabase** API and outputs the results as formatted Markdown.

## Features

- Look up any IP address or omit the argument to query your own public IP
- Retrieves geolocation (continent, country, region, city, coordinates, postal code)
- Connection details (ASN, organization, connection type)
- Time zone information (ID, abbreviation, UTC offset)
- Currency details (code, name, symbol)
- Security flags (VPN, proxy, Tor, threat detection)
- API key authentication via `config.json`
- Clean Markdown-formatted output

## Installation

### From source

```bash
git clone https://github.com/kleedodo/ipinfo.git
cd ipinfo
cargo install --path .
```

This builds and installs the binary to your Cargo bin directory (typically `~/.cargo/bin`). Make sure `~/.cargo/bin` is in your `PATH`.

Alternatively, you can build manually:

```bash
git clone https://github.com/kleedodo/ipinfo.git
cd ipinfo
cargo build --release
```

The binary will be available at `target/release/ipinfo`.

### Pre-built binaries

Pre-built binaries for various platforms are available on the [GitHub Releases](https://github.com/kleedodo/ipinfo/releases) page. Download the appropriate archive for your platform, extract it, and place the `ipinfo` binary in a directory on your `PATH`.

## Usage

```bash
# Look up your own public IP
ipinfo

# Look up a specific IP address
ipinfo 8.8.8.8
```

### Configuration

Create a `config.json` file in the current working directory (or `~/.config/ipinfo/config.json` for a global configuration):

```json
{
  "providers": {
    "dkly": {
      "key": "your-api-key"
    }
  }
}
```

The local `./config.json` takes precedence over `~/.config/ipinfo/config.json`. See `config.json.example` for reference.

### Example Output

```markdown
# IP Information: 8.8.8.8

## Basic Info
- **IP**: 8.8.8.8
- **Hostname**: dns.google

## Connection
- **ASN**: 15169
- **Organization**: Google LLC

## Location
- **Continent**: North America (NA)
- **Country**: United States (US) 🇺🇸
- **Region**: California (CA)
- **City**: Mountain View
- **Postal**: 94043
- **Coordinates**: 37.386, -122.0838

## Time Zone
- **ID**: America/Los_Angeles
- **Abbreviation**: PDT
- **Offset**: -25200 seconds (UTC)

## Currency
- **Code**: USD
- **Name**: United States Dollar
- **Symbol**: $

## Security
- **VPN**: No
- **Proxy**: No
- **Tor**: No
- **Threat**: No
```

## API Reference

The tool queries the dklyIPdatabase API at `https://ipinfo.dkly.net/api/`. The API accepts the following query parameters:

| Parameter | Description |
|-----------|-------------|
| `key`     | API key (optional if set in config.json) |
| `ip`      | IP address to look up (optional; defaults to requester's IP) |

## Dependencies

Key crates used:

- **clap** — CLI argument parsing (derive API)
- **reqwest** — HTTP client (blocking, JSON, rustls-tls)
- **serde / serde_json** — JSON serialization / deserialization
- **anyhow** — Error handling

## Agent Skill

This project includes an Agent Skill at `.agents/skills/ipinfo/SKILL.md` following the [Agent Skills](https://agentskills.io/) open standard. It provides IP geolocation and network intelligence to any skills-compatible AI agent (Claude Code, Cursor, OpenAI Codex, etc.).

## License

See the [LICENSE](LICENSE) file for details.
