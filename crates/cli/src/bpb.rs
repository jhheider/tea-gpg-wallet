use std::process::Command;

use anyhow::{Context, Result, anyhow};
use lazy_static::lazy_static;
use libtea_gpg_wallet::wallet::SigningData;
use regex::Regex;

use crate::utils::SigningResult;

lazy_static! {
    static ref BPB_OUTPUT: Regex =
        Regex::new(r"^signature:\n+([0-9a-fA-F]+)\n+public key:\n+([0-9a-fA-F]+)\n*$").unwrap();
}

pub async fn get_key_id() -> Result<String> {
    let output = Command::new("bpb")
        .arg("key-id")
        .output()
        .context("Failed to get key ID")?;

    if !output.status.success() {
        return Err(anyhow!("bpb command failed with status: {}", output.status));
    }

    let key_id = String::from_utf8(output.stdout)
        .context("Failed to convert key ID to string")?
        .trim()
        .to_string();

    if key_id.is_empty() {
        return Err(anyhow!("Key ID is empty"));
    }

    let key_id = key_id
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<String>();

    Ok(key_id)
}

pub async fn sign_blob(signing_data: &SigningData) -> Result<SigningResult> {
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
    let pubkey = captures
        .get(2)
        .context("Failed to get public key from captures")?
        .as_str();
    Ok(SigningResult {
        signature: signature.to_string(),
        public_key: pubkey.to_string(),
    })
}
