use alloy::primitives::U256;
use anyhow::{Context, Result};
use colored::{ColoredString, Colorize};

pub const ETH_DECIMALS: usize = 18;

pub struct SigningResult {
    pub signature: String,
    pub public_key: String,
}

// Removes all non-hex characters from a string
pub fn filter_hex_string(key_id: &str) -> String {
    key_id
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<String>()
}

// Colors a hex string in rotating colors by chunk, skipping leading "0x" if present
pub fn hex_color(s: impl AsRef<str>, chunk_size: usize) -> Result<ColoredString> {
    let s = s.as_ref();
    let colors = [
        Colorize::red,
        Colorize::green,
        Colorize::yellow,
        Colorize::blue,
        Colorize::magenta,
        Colorize::cyan,
    ];
    let mut colored = String::new();
    let s = if s.starts_with("0x") {
        colored.push_str("0x");
        s.strip_prefix("0x")
            .context("Failed to strip '0x' prefix")?
    } else {
        s
    };

    for (i, chunk) in s.as_bytes().chunks(chunk_size).enumerate() {
        if i == 0 && chunk == b"0x" {
            colored.push_str("0x");
            continue;
        }
        let color = colors[i % colors.len()];
        let part = std::str::from_utf8(chunk).unwrap_or("");
        colored.push_str(&color(part).to_string());
    }
    Ok(colored.normal())
}

pub fn require_private_key() -> Result<String> {
    std::env::var("PRIVATE_KEY").context("PRIVATE_KEY environment variable not set")
}

/// Convert wei to ETH with automatic decimal formatting
/// Removes trailing zeros and unnecessary decimal places
pub fn wei_to_eth_auto(wei: U256) -> String {
    let wei_str = wei.to_string();

    if wei_str.len() <= ETH_DECIMALS {
        // Pad with leading zeros if needed
        let mut padded = wei_str.clone();
        while padded.len() < ETH_DECIMALS {
            padded.insert(0, '0');
        }

        // Insert decimal point
        if padded.len() == ETH_DECIMALS {
            padded.insert(0, '0');
        }
        padded.insert(padded.len() - ETH_DECIMALS, '.');

        // Remove trailing zeros
        padded
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        // Large number, split at the right position
        let mut result = wei_str.clone();
        result.insert(wei_str.len() - ETH_DECIMALS, '.');

        // Remove trailing zeros
        result
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}
