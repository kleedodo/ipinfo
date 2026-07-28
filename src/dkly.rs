use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const BASE_URL: &str = "https://ipinfo.dkly.net/api/";

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// Top-level configuration loaded from `config.json`.
#[derive(serde::Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub providers: Providers,
}

/// Provider registry.
#[derive(serde::Deserialize, Debug, Default)]
pub struct Providers {
    #[serde(default)]
    pub dkly: DklyProvider,
}

/// dkly provider configuration.
#[derive(serde::Deserialize, Debug, Default)]
pub struct DklyProvider {
    #[serde(default)]
    pub key: Option<String>,
}

impl Config {
    /// Load configuration. Searches, in order:
    ///  1. `./config.json` (local, takes precedence)
    ///  2. `~/.config/ipinfo/config.json` (global)
    ///
    /// Returns `Ok(None)` when no config file is found.
    pub fn load() -> Result<Option<Self>> {
        // 1. Local config (current working directory)
        let local = PathBuf::from("config.json");
        if local.exists() {
            let content = std::fs::read_to_string(&local)?;
            let config: Self = serde_json::from_str(&content)?;
            return Ok(Some(config));
        }

        // 2. Global config (~/.config/ipinfo/config.json)
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let global = PathBuf::from(home)
                .join(".config")
                .join("ipinfo")
                .join("config.json");
            if global.exists() {
                let content = std::fs::read_to_string(&global)?;
                let config: Self = serde_json::from_str(&content)?;
                return Ok(Some(config));
            }
        }

        Ok(None)
    }
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
}

// ---------------------------------------------------------------------------
// Response structures
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct ApiResponse {
    ip: String,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    hostname: Option<String>,
    #[serde(default)]
    connection: Option<Connection>,
    #[serde(default)]
    location: Option<Location>,
    #[serde(default)]
    time_zone: Option<TimeZone>,
    #[serde(default)]
    currency: Option<Currency>,
    #[serde(default)]
    security: Option<Security>,
}

#[derive(Deserialize)]
struct Connection {
    #[serde(default)]
    asn: Option<u32>,
    #[serde(default)]
    organization: Option<String>,
    #[serde(default)]
    r#type: Option<String>,
}

#[derive(Deserialize)]
struct Location {
    #[serde(default)]
    continent: Option<Continent>,
    #[serde(default)]
    country: Option<Country>,
    #[serde(default)]
    region: Option<Region>,
    #[serde(default)]
    city: Option<String>,
    #[serde(default)]
    postal: Option<String>,
    #[serde(default)]
    latitude: Option<f64>,
    #[serde(default)]
    longitude: Option<f64>,
}

#[derive(Deserialize)]
struct Continent {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Deserialize)]
struct Country {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    flag: Option<Flag>,
}

#[derive(Deserialize)]
struct Flag {
    #[serde(default)]
    emoji: Option<String>,
}

#[derive(Deserialize)]
struct Region {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Deserialize)]
struct TimeZone {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    abbreviation: Option<String>,
    #[serde(default)]
    offset: Option<i64>,
}

#[derive(Deserialize)]
struct Currency {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    symbol: Option<String>,
}

#[derive(Deserialize)]
struct Security {
    #[serde(default)]
    is_vpn: bool,
    #[serde(default)]
    is_proxy: bool,
    #[serde(default)]
    is_tor: bool,
    #[serde(default)]
    is_threat: bool,
}

#[derive(Deserialize)]
struct ApiError {
    code: String,
    message: String,
    #[serde(default)]
    resolution: Option<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn query(ip: Option<String>, api_key: Option<String>) -> Result<()> {
    let params = QueryParams { key: api_key, ip };

    let client = Client::new();

    let response = client.get(BASE_URL).query(&params).send()?;
    let body = response.text()?;

    if let Ok(data) = serde_json::from_str::<ApiResponse>(&body) {
        print_markdown(&data);
    } else if let Ok(err) = serde_json::from_str::<ApiError>(&body) {
        print_error_markdown(&err);
    } else {
        println!("```json\n{body}\n```");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Markdown output
// ---------------------------------------------------------------------------

fn print_error_markdown(err: &ApiError) {
    println!("# API Error");
    println!();
    println!("- **Code**: {}", err.code);
    println!("- **Message**: {}", err.message);
    if let Some(res) = &err.resolution {
        println!("- **Resolution**: {res}");
    }
}

fn print_markdown(data: &ApiResponse) {
    println!("# IP Information: {}", data.ip);
    println!();

    // Basic Info
    println!("## Basic Info");
    println!("- **IP**: {}", data.ip);
    if let Some(t) = &data.r#type {
        println!("- **Type**: {t}");
    }
    if let Some(h) = &data.hostname {
        println!("- **Hostname**: {h}");
    }
    println!();

    // Connection
    if let Some(conn) = &data.connection {
        println!("## Connection");
        if let Some(asn) = conn.asn {
            println!("- **ASN**: {asn}");
        }
        if let Some(org) = &conn.organization {
            println!("- **Organization**: {org}");
        }
        if let Some(t) = &conn.r#type {
            println!("- **Type**: {t}");
        }
        println!();
    }

    // Location
    if let Some(loc) = &data.location {
        println!("## Location");
        if let Some(cont) = &loc.continent {
            let name = cont.name.as_deref().unwrap_or("N/A");
            let code = cont.code.as_deref().unwrap_or("N/A");
            println!("- **Continent**: {name} ({code})");
        }
        if let Some(c) = &loc.country {
            let name = c.name.as_deref().unwrap_or("N/A");
            let code = c.code.as_deref().unwrap_or("N/A");
            let flag = c
                .flag
                .as_ref()
                .and_then(|f| f.emoji.as_deref())
                .unwrap_or("");
            println!("- **Country**: {name} ({code}) {flag}");
        }
        if let Some(r) = &loc.region {
            let name = r.name.as_deref().unwrap_or("N/A");
            let code = r.code.as_deref().unwrap_or("N/A");
            println!("- **Region**: {name} ({code})");
        }
        if let Some(city) = &loc.city {
            println!("- **City**: {city}");
        }
        if let Some(postal) = &loc.postal {
            println!("- **Postal**: {postal}");
        }
        if let (Some(lat), Some(lon)) = (loc.latitude, loc.longitude) {
            println!("- **Coordinates**: {lat}, {lon}");
        }
        println!();
    }

    // Time Zone
    if let Some(tz) = &data.time_zone {
        println!("## Time Zone");
        if let Some(id) = &tz.id {
            println!("- **ID**: {id}");
        }
        if let Some(abbr) = &tz.abbreviation {
            println!("- **Abbreviation**: {abbr}");
        }
        if let Some(offset) = tz.offset {
            println!("- **Offset**: {offset} seconds (UTC)");
        }
        println!();
    }

    // Currency
    if let Some(cur) = &data.currency {
        println!("## Currency");
        if let Some(code) = &cur.code {
            println!("- **Code**: {code}");
        }
        if let Some(name) = &cur.name {
            println!("- **Name**: {name}");
        }
        if let Some(symbol) = &cur.symbol {
            println!("- **Symbol**: {symbol}");
        }
        println!();
    }

    // Security
    if let Some(sec) = &data.security {
        println!("## Security");
        println!("- **VPN**: {}", if sec.is_vpn { "Yes" } else { "No" });
        println!("- **Proxy**: {}", if sec.is_proxy { "Yes" } else { "No" });
        println!("- **Tor**: {}", if sec.is_tor { "Yes" } else { "No" });
        println!("- **Threat**: {}", if sec.is_threat { "Yes" } else { "No" });
    }
}
