use reqwest::Url as URL;
use std::str::FromStr;

use alloy::primitives::FixedBytes;
use anyhow::{Context, Result};

#[inline]
pub fn get_rpc_url() -> Result<URL> {
    env!("RPC_URL").parse().context("Invalid RPC URL")
}

#[inline]
pub fn key_id_to_bytes(key_id: &str) -> Result<FixedBytes<8>> {
    FixedBytes::from_str(key_id).context("Failed to convert key id to FixedBytes")
}
