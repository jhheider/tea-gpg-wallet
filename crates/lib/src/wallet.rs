use alloy::{
    primitives::{Address, Bytes, FixedBytes, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{Context, Result};
use std::{str::FromStr, time::SystemTime};

use crate::{deployer::predict_address, utils::get_rpc_url};

sol!(
    #[allow(clippy::too_many_arguments)]
    #[sol(rpc)]
    GpgRewardWallet,
    "abi/GpgRewardWallet.json"
);

pub struct SigningData {
    pub blob: FixedBytes<32>,
    pub deadline: U256,
}

pub async fn get_signable_hash(key_id: &str, to: &str) -> Result<SigningData> {
    let provider = ProviderBuilder::new().connect_http(get_rpc_url()?);
    let destination = predict_address(key_id).await?;
    if !destination.isDeployed {
        panic!("GPG wallet for key ID {key_id} is not deployed");
    }
    let wallet = GpgRewardWallet::new(destination.walletAddress, provider);
    let nonce = wallet
        .nextNonce()
        .call()
        .await
        .context("Failed to get nonce")?;
    let deadline = U256::from(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("Failed to get current time")?
            .as_secs()
            + 120,
    ); // 2 minutes from now
    let to = Address::from_str(to).context("Failed to parse destination address")?;
    let blob = wallet
        .getWithdrawAllStructHash(to, U256::ZERO, deadline, nonce)
        .call()
        .await
        .context("Failed to get signable hash")?;
    Ok(SigningData { blob, deadline })
}

pub async fn sweep_gpg_key(
    key_id: &str,
    to: &str,
    deadline: U256,
    public_key: &str,
    signature: &str,
    private_key: &str,
) -> Result<FixedBytes<32>> {
    let provider = ProviderBuilder::new()
        .wallet(
            private_key
                .parse::<PrivateKeySigner>()
                .context("Invalid private key")?,
        )
        .connect_http(get_rpc_url()?);
    let destination = predict_address(key_id).await?;
    if !destination.isDeployed {
        panic!("GPG wallet for key ID {key_id} is not deployed");
    }
    let wallet = GpgRewardWallet::new(destination.walletAddress, provider);
    let to = Address::from_str(to).context("Failed to parse destination address")?;
    let pubkey =
        Bytes::from_str(public_key).context("Failed to convert public key to FixedBytes")?;
    let signature =
        Bytes::from_str(signature).context("Failed to convert signature to FixedBytes")?;
    Ok(wallet
        .withdrawAll(to, U256::ZERO, deadline, pubkey, signature)
        .send()
        .await
        .context("Failed to send withdrawal transaction")?
        .get_receipt()
        .await
        .context("Withdrawal transaction failed")?
        .transaction_hash)
}

#[cfg(test)]
mod tests {
    use crate::wallet::get_signable_hash;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_signable_hash() -> Result<()> {
        let key_id = "95469C7E3DFC90B1";
        let to = "0x590b78eaF98053eFBa4107Eed2e0F70D2B90A45d";
        get_signable_hash(key_id, to).await?;
        Ok(())
    }
}
