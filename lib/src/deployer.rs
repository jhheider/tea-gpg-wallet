use crate::utils::{get_rpc_url, key_id_to_bytes};
use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol!(
    #[sol(rpc)]
    GpgRewardDeployer,
    "abi/GpgRewardDeployer.json"
);

pub fn get_contract_address() -> Address {
    Address::parse_checksummed(env!("GPG_DEPLOYER_ADDRESS"), None)
        .expect("Invalid GPG deployer address")
}

pub async fn predict_address(key_id: &str) -> GpgRewardDeployer::predictAddressReturn {
    let provider = ProviderBuilder::new().connect_http(get_rpc_url());
    let key_id = key_id_to_bytes(key_id);
    GpgRewardDeployer::new(get_contract_address(), provider)
        .predictAddress(key_id)
        .call()
        .await
        .expect("Failed to predict address")
}

pub async fn ensure_deployed(key_id: &str) -> GpgRewardDeployer::predictAddressReturn {
    let prediction = predict_address(key_id).await;

    if prediction.isDeployed {
        return prediction;
    }
    let provider = ProviderBuilder::new().connect_http(get_rpc_url());
    GpgRewardDeployer::new(get_contract_address(), provider)
        .deploy(key_id_to_bytes(key_id))
        .send()
        .await
        .expect("Failed to deploy GPG reward wallet")
        .get_receipt()
        .await
        .expect("Deployment transaction failed");

    predict_address(key_id).await
}

pub async fn get_key_id_balance(key_id: &str) -> U256 {
    let provider = ProviderBuilder::new().connect_http(get_rpc_url());
    let destination = ensure_deployed(key_id).await;
    provider
        .get_balance(destination.walletAddress)
        .await
        .expect("Failed to get balance")
}

// send to a gpg wallet, confirming and deploying as necessary
// key_id: the GPG key id, e.g. "95469C7E3DFC90B1"
// amount: the amount to send in wei
// private_key: the private key of the sender
// returns balance
pub async fn send_to_gpg_key(key_id: &str, amount: U256, private_key: &str) -> U256 {
    let provider = ProviderBuilder::new()
        .wallet(
            private_key
                .parse::<PrivateKeySigner>()
                .expect("Invalid private key"),
        )
        .connect_http(env!("RPC_URL").parse().expect("Invalid RPC URL"));
    let destination = ensure_deployed(key_id).await;

    let send = TransactionRequest::default()
        .to(destination.walletAddress)
        .value(amount);

    let receipt = provider
        .send_transaction(send)
        .await
        .expect("Failed to send transaction")
        .get_receipt()
        .await
        .expect("Transaction failed");

    if !receipt.status() {
        panic!("Transaction failed with status: {:?}", receipt.status());
    }

    get_key_id_balance(key_id).await
}

#[cfg(test)]
mod tests {
    use std::env;

    use alloy::primitives::address;

    use super::*;

    #[tokio::test]
    async fn test_predict_address() {
        let predicted_address = predict_address("95469C7E3DFC90B1").await;
        assert_eq!(
            predicted_address.walletAddress,
            address!("0xd7baae85d719c2e8e27a70194471ef4b6b253d33")
        );
    }

    #[tokio::test]
    async fn test_ensure_deployed() {
        // already deployed
        let key_id = "95469C7E3DFC90B1";
        let prediction = ensure_deployed(key_id).await;
        assert!(prediction.isDeployed);
        assert_eq!(
            prediction.walletAddress,
            address!("0xd7baae85d719c2e8e27a70194471ef4b6b253d33")
        );
    }

    #[tokio::test]
    async fn test_get_key_id_balance() {
        let balance = get_key_id_balance("95469C7E3DFC90B1").await;
        eprintln!("Balance: {balance}");
    }

    #[tokio::test]
    async fn test_send_to_id() {
        let Ok(pk) = env::var("PRIVATE_KEY") else {
            eprintln!("PRIVATE_KEY environment variable not set\nSkipping test_send_to_id");
            return;
        };
        let key_id = "95469C7E3DFC90B1";
        let amount = U256::from(1_000_000_000_000_000u64); // 1 Gwei

        let stating_balance = get_key_id_balance(key_id).await;
        let new_balance = send_to_gpg_key(key_id, amount, &pk).await;
        assert_eq!(new_balance, stating_balance + amount);
    }
}
