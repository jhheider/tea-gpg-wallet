use std::process::Command;

use alloy::primitives::U256;
use anyhow::{Context, Ok, Result};
use lazy_static::lazy_static;
use libtea_gpg_wallet::{
    deployer::{self, get_key_id_balance, predict_address, send_to_gpg_key},
    wallet::{get_signable_hash, sweep_gpg_key},
};
use regex::Regex;

lazy_static! {
    static ref BPB_OUTPUT: Regex =
        Regex::new(r"^signature:\n+([0-9a-fA-F]+)\n+public key:\n+([0-9a-fA-F]+)\n*$").unwrap();
}

// Testing
const TO_ADDRESS: &str = "0x590b78eaF98053eFBa4107Eed2e0F70D2B90A45d";

#[tokio::main]
async fn main() -> Result<()> {
    // Require private key
    let private_key =
        std::env::var("PRIVATE_KEY").context("PRIVATE_KEY environment variable not set")?;

    // Test info functions
    println!(
        "gpg deployer address: {}",
        deployer::get_contract_address()?
    );

    // Get key ID from bpb
    let key_id = Command::new("bpb")
        .arg("key-id")
        .output()
        .context("Failed to get key ID")?
        .stdout
        .into_iter()
        .filter(|&b| b != b' ' && b != b'\n') // Remove spaces
        .collect();
    let key_id = String::from_utf8(key_id).context("Failed to convert key ID to string")?;
    println!("key id: {key_id}");

    // Predict wallet address from key_id
    let predicted_address = predict_address(&key_id).await?;
    println!(
        "predicted address for key id {key_id}: {:?}, is_deployed: {}",
        predicted_address.walletAddress, predicted_address.isDeployed
    );

    // Send 1 Gwei to the key ID, creating the wallet if necessary
    let balance = get_key_id_balance(&key_id).await?;
    println!("balance for key id {key_id}: {balance}");

    // Confirm the send
    let new_balance =
        send_to_gpg_key(&key_id, U256::from(1_000_000_000_000_000u64), &private_key).await?;
    println!("new balance for key id {key_id}: {new_balance}, was: {balance}");

    // Get the signable hash for a sweep
    let signing_data = get_signable_hash(&key_id, TO_ADDRESS).await?;
    println!(
        "signable hash for key id {key_id}: {:?}",
        signing_data.blob.to_string()
    );

    // Sign the hash using bpb
    let signature = Command::new("bpb")
        .arg("sign-hex")
        .arg(signing_data.blob.to_string())
        .output()
        .context("Failed to sign hash")?
        .stdout;
    let signature =
        String::from_utf8(signature).context("Failed to convert signature to string")?;
    let captures = BPB_OUTPUT
        .captures(&signature)
        .context("Failed to capture signature and public key")?;
    let signature = captures
        .get(1)
        .context("Failed to get signature from captures")?
        .as_str();
    let public_key = captures
        .get(2)
        .context("Failed to get public key from captures")?
        .as_str();
    println!("signature: {signature}");
    println!("public key: {public_key}");

    // Sweep the wallet
    let tx_hash = sweep_gpg_key(
        &key_id,
        TO_ADDRESS,
        signing_data.deadline,
        public_key,
        signature,
        &private_key,
    )
    .await?;
    println!("sweep transaction hash for key id {key_id}: {tx_hash}");

    // Confirm the sweep
    let new_balance = get_key_id_balance(&key_id).await?;
    println!("new balance for key id {key_id} after sweep: {new_balance}, was: {balance}");

    Ok(())
}
