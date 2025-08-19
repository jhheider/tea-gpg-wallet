use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

use alloy::hex;
use anyhow::{Context, Result, anyhow};
use libtea_gpg_wallet::wallet::SigningData;

use crate::utils::{SigningResult, filter_hex_string};

// Retrieves the GPG key ID by executing the `gpg --list-keys --with-colons` command
// for a given email address.
pub async fn get_key_id(email: &str) -> Result<String> {
    let output = Command::new("gpg")
        .arg("--list-keys")
        .arg("--with-colons")
        .output()
        .context("Failed to get GPG key ID")?;

    if !output.status.success() {
        return Err(anyhow!("gpg command failed with status: {}", output.status));
    }

    let output_str = String::from_utf8(output.stdout)
        .context("Failed to convert GPG key ID output to string")?;

    let key_id: String = {
        let mut temp = String::new();
        for line in output_str.lines() {
            if line.starts_with("pub:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 4 {
                    temp = parts[4].trim().to_string();
                }
            } else if line.starts_with("uid:") && line.contains(&format!("<{email}>")) {
                return Ok(temp);
            }
        }
        Err(anyhow!("No GPG key found for email: {}", email))
    }?;

    if key_id.is_empty() {
        return Err(anyhow!("GPG key ID is empty"));
    }

    let key_id = filter_hex_string(&key_id);

    if key_id.is_empty() {
        return Err(anyhow!("GPG key ID is empty after filtering"));
    }

    Ok(key_id)
}

pub async fn sign_blob(signing_data: &SigningData, key_id: &str) -> Result<SigningResult> {
    // echo "0x8941bd5962cdb275a3f5f1ffa623aa3be1fc40f55b0b308ab906cf9f7ef39cac" | xxd -r -p | gpg -u 95469C7E3DFC90B1 --pinentry-mode loopback --detach-sign | xxd -p | tr -d '\n'
    let mut proc = Command::new("gpg")
        .arg("-u")
        .arg(key_id)
        .arg("--pinentry-mode")
        .arg("loopback")
        .arg("--detach-sign")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn GPG process")?;

    let mut stdin = proc.stdin.take().expect("Failed to open stdin");
    let mut stdout = proc.stdout.take().context("Failed to sign blob with GPG")?;

    stdin
        .write_all(signing_data.blob.as_slice())
        .context("Failed to write to stdin")?;
    drop(stdin); // Close stdin to signal EOF

    let mut buffer = Vec::new();

    stdout.read_to_end(&mut buffer)?;
    let status = proc.wait().context("Failed to wait for GPG process")?;

    if !status.success() {
        return Err(anyhow!("GPG signing failed with status: {}", status));
    }

    let signature = hex::encode(buffer);

    // gpg --export 95469C7E3DFC90B1 | xxd -p | tr -d '\n'
    let pubkey = Command::new("gpg")
        .arg("--export")
        .arg(key_id)
        .output()
        .context("Failed to sign hash")?
        .stdout;

    let public_key = hex::encode(pubkey);

    Ok(SigningResult {
        signature,
        public_key,
    })
}
