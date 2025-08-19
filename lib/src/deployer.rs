use crate::utils::{get_rpc_url, key_id_to_bytes};
use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{Context, Result, anyhow};

sol!(
    #[sol(rpc)]
    GpgRewardDeployer,
    "abi/GpgRewardDeployer.json"
);

pub fn get_contract_address() -> Result<Address> {
    Address::parse_checksummed(env!("GPG_DEPLOYER_ADDRESS"), None)
        .context("Invalid GPG deployer address configured at build time.")
}

pub async fn predict_address(key_id: &str) -> Result<GpgRewardDeployer::predictAddressReturn> {
    let provider = ProviderBuilder::new().connect_http(get_rpc_url()?);
    let key_id = key_id_to_bytes(key_id)?;
    GpgRewardDeployer::new(get_contract_address()?, provider)
        .predictAddress(key_id)
        .call()
        .await
        .context("Failed to predict address")
}

pub async fn ensure_deployed(
    key_id: &str,
    private_key: &str,
) -> Result<GpgRewardDeployer::predictAddressReturn> {
    let prediction = predict_address(key_id).await?;

    if prediction.isDeployed {
        return Ok(prediction);
    }
    let provider = ProviderBuilder::new()
        .wallet(
            private_key
                .parse::<PrivateKeySigner>()
                .context("Invalid private key")?,
        )
        .connect_http(get_rpc_url()?);
    GpgRewardDeployer::new(get_contract_address()?, provider)
        .deploy(key_id_to_bytes(key_id)?)
        .send()
        .await
        .context("Failed to deploy GPG reward wallet")?
        .get_receipt()
        .await
        .context("Deployment transaction failed")?;

    predict_address(key_id).await
}

pub async fn get_key_id_balance(key_id: &str) -> Result<U256> {
    let provider = ProviderBuilder::new().connect_http(get_rpc_url()?);
    let destination = predict_address(key_id).await?;
    if !destination.isDeployed {
        return Err(anyhow!("GPG wallet for key ID {key_id} is not deployed"));
    }
    provider
        .get_balance(destination.walletAddress)
        .await
        .context("Failed to get balance")
}

// send to a gpg wallet, confirming and deploying as necessary
// key_id: the GPG key id, e.g. "95469C7E3DFC90B1"
// amount: the amount to send in wei
// private_key: the private key of the sender
// returns balance
pub async fn send_to_gpg_key(key_id: &str, amount: U256, private_key: &str) -> Result<U256> {
    let provider = ProviderBuilder::new()
        .wallet(
            private_key
                .parse::<PrivateKeySigner>()
                .context("Invalid private key")?,
        )
        .connect_http(get_rpc_url()?);
    let destination = ensure_deployed(key_id, private_key).await?;

    let send = TransactionRequest::default()
        .to(destination.walletAddress)
        .value(amount);

    let receipt = provider
        .send_transaction(send)
        .await
        .context("Failed to send transaction")?
        .get_receipt()
        .await
        .context("Transaction failed")?;

    if !receipt.status() {
        return Err(anyhow!(
            "Transaction failed with status: {:?}",
            receipt.status()
        ));
    }

    get_key_id_balance(key_id).await
}

#[cfg(test)]
mod tests {
    use std::env;

    use alloy::primitives::address;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_predict_address() -> Result<()> {
        let predicted_address = predict_address("95469C7E3DFC90B1").await?;
        assert_eq!(
            predicted_address.walletAddress,
            address!("0xd7baae85d719c2e8e27a70194471ef4b6b253d33")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_ensure_deployed() -> Result<()> {
        let Ok(pk) = env::var("PRIVATE_KEY") else {
            eprintln!("PRIVATE_KEY environment variable not set\nSkipping test_send_to_id");
            return Ok(());
        };
        // already deployed
        let key_id = "95469C7E3DFC90B1";
        let prediction = ensure_deployed(key_id, &pk).await?;
        assert!(prediction.isDeployed);
        assert_eq!(
            prediction.walletAddress,
            address!("0xd7baae85d719c2e8e27a70194471ef4b6b253d33")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_key_id_balance() -> Result<()> {
        let balance = get_key_id_balance("95469C7E3DFC90B1").await?;
        eprintln!("Balance: {balance}");
        Ok(())
    }

    #[tokio::test]
    async fn test_send_to_id() -> Result<()> {
        let Ok(pk) = env::var("PRIVATE_KEY") else {
            eprintln!("PRIVATE_KEY environment variable not set\nSkipping test_send_to_id");
            return Ok(());
        };
        let key_id = "95469C7E3DFC90B1";
        let amount = U256::from(1_000_000_000_000_000u64); // 1 Gwei

        let starting_balance = get_key_id_balance(key_id).await?;
        let new_balance = send_to_gpg_key(key_id, amount, &pk).await?;
        assert_eq!(new_balance, starting_balance + amount);
        Ok(())
    }
}
