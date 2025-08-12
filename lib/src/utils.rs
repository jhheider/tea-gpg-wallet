use reqwest::Url as URL;
use std::str::FromStr;

use alloy::primitives::FixedBytes;

#[inline]
pub fn get_rpc_url() -> URL {
    env!("RPC_URL").parse().expect("Invalid RPC URL")
}

#[inline]
pub fn key_id_to_bytes(key_id: &str) -> FixedBytes<8> {
    FixedBytes::from_str(key_id).expect("Failed to convert key id to FixedBytes")
}
