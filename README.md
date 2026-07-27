# ipinfo

A Rust CLI tool that queries IP information from the **dklyIPdatabase** API and outputs the results as formatted Markdown.

## Features

- Look up any IP address or omit the argument to query your own public IP
- Retrieves geolocation (continent, country, region, city, coordinates, postal code)
- Connection details (ASN, organization, connection type)
- Time zone information (ID, abbreviation, UTC offset)
- Currency details (code, name, symbol)
- Security flags (VPN, proxy, Tor, threat detection)
- API key authentication via environment variable
- Clean Markdown-formatted output

## Installation

```bash
cargo build --release
```

Or clone the repository and build:

```bash
git clone https://github.com/<your-org>/ipinfo.git
cd ipinfo
cargo build --release
```

The binary will be available at `target/release/ipinfo`.

## Usage

```bash
# Look up your own public IP
ipinfo

# Look up a specific IP address
ipinfo 8.8.8.8
```

### Configuration

Set your API key via the `DKLY_API_KEY` environment variable. The tool will read a `.env` file automatically if one is present.

```bash
export DKLY_API_KEY="your-api-key"
```

Or create a `.env` file in the project root:

```env
DKLY_API_KEY=your-api-key
```

> **Note:** The `.env` file is not committed to version control. See `.gitignore` and `.env.example` for reference.

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
| `key`     | API key (optional if set via env var) |
| `ip`      | IP address to look up (optional; defaults to requester's IP) |

## Dependencies

Key crates used:

- **clap** — CLI argument parsing (derive API)
- **reqwest** — HTTP client (blocking, JSON, rustls-tls)
- **serde / serde_json** — JSON serialization / deserialization
- **dotenvy** — `.env` file loading
- **anyhow** — Error handling

## License

See the LICENSE file for details.
