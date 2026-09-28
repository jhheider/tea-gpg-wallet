use reqwest::Url as URL;
use std::str::FromStr;

use alloy::primitives::{FixedBytes, U256};
use anyhow::{Context, Result, anyhow};

pub const WEI_PER_ETH: u128 = 1_000_000_000_000_000_000;
pub const ETH_DECIMALS: usize = 18;

#[inline]
pub fn get_rpc_url() -> Result<URL> {
    env!("RPC_URL").parse().context("Invalid RPC URL")
}

#[inline]
pub fn key_id_to_bytes(key_id: &str) -> Result<FixedBytes<8>> {
    FixedBytes::from_str(key_id).context("Failed to convert key id to FixedBytes")
}

/// Convert a decimal string to U256 representing wei with higher precision
/// This method handles larger amounts accurately
pub fn decimal_to_wei_precise(amount_str: &str) -> Result<U256> {
    // Split on decimal point
    let parts: Vec<&str> = amount_str.split('.').collect();

    match parts.as_slice() {
        [whole] => {
            // No decimal part, just multiply by 10^18
            let whole_u256 =
                U256::from_str_radix(whole, 10).context("Failed to parse whole number")?;
            Ok(whole_u256 * U256::from(WEI_PER_ETH))
        }
        [whole, decimal] => {
            // Has decimal part
            let whole_u256 =
                U256::from_str_radix(whole, 10).context("Failed to parse whole number")?;

            // Pad decimal part to 18 digits
            let mut padded_decimal = decimal.to_string();
            while padded_decimal.len() < ETH_DECIMALS {
                padded_decimal.push('0');
            }
            if padded_decimal.len() > ETH_DECIMALS {
                padded_decimal.truncate(ETH_DECIMALS);
            }

            let decimal_u256 = U256::from_str_radix(&padded_decimal, 10)
                .context("Failed to parse decimal part")?;

            Ok(whole_u256 * U256::from(WEI_PER_ETH) + decimal_u256)
        }
        _ => Err(anyhow!("Invalid decimal format")),
    }
}
