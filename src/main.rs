use std::env;

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

mod dkly;

/// Query IP information from the dklyIPdatabase API.
///
/// If no IP address is provided, the requester's public IP is looked up.
/// The API key is read from the DKLY_API_KEY environment variable.
/// When no key is set, requests are made without authentication.
#[derive(Parser)]
#[command(name = "ipinfo", version, about)]
struct Args {
    /// IP address to look up (omit to query your public IP)
    ip: Option<String>,
}

fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();
    let api_key = env::var(dkly::ENV_API_KEY).ok();

    dkly::query(args.ip, api_key)
}
