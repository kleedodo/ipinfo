use anyhow::Result;
use clap::Parser;

mod dkly;

/// Query IP information from the dklyIPdatabase API.
///
/// If no IP address is provided, the requester's public IP is looked up.
/// The API key is read from `config.json` (local or `~/.config/ipinfo/config.json`).
/// When no key is set, requests are made without authentication.
#[derive(Parser)]
#[command(name = "ipinfo", version, about)]
struct Args {
    /// IP address to look up (omit to query your public IP)
    ip: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = dkly::Config::load()?;
    let api_key = config.as_ref().and_then(|c| c.providers.dkly.key.clone());

    dkly::query(args.ip, api_key)
}
