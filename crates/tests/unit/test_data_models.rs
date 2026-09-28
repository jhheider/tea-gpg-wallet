//! Unit tests for data model validation
//!
//! These tests validate data model validation rules.
//! Tests must fail initially (TDD approach) and will pass once implementation is complete.

use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use anyhow::Result;
use libtea_gpg_wallet::models::{AddressPrediction, SigningData, SigningResult, WalletStatus};

#[test]
fn test_address_prediction_validation() -> Result<()> {
    // Test valid AddressPrediction
    let valid_address = "0xd7baae85d719c2e8e27a70194471ef4b6b253d33".parse::<Address>()?;
    let prediction = AddressPrediction {
        wallet_address: valid_address,
        is_deployed: true,
    };

    assert_eq!(
        prediction.wallet_address.to_string().len(),
        42,
        "Address should be 42 characters"
    );
    assert!(
        prediction.wallet_address.to_string().starts_with("0x"),
        "Address should start with 0x"
    );
    assert_eq!(
        prediction.is_deployed, true,
        "Deployment status should match"
    );

    // Test with not deployed
    let prediction_not_deployed = AddressPrediction {
        wallet_address: valid_address,
        is_deployed: false,
    };

    assert_eq!(
        prediction_not_deployed.is_deployed, false,
        "Not deployed status should match"
    );

    Ok(())
}

#[test]
fn test_signing_data_withdraw_variant() -> Result<()> {
    // Test SigningData::Withdraw variant
    let blob = FixedBytes::<32>::from([0u8; 32]);
    let deadline = U256::from(1735689600u64); // Future timestamp

    let signing_data = SigningData::Withdraw { blob, deadline };

    match signing_data {
        SigningData::Withdraw {
            blob: b,
            deadline: d,
        } => {
            assert_eq!(b.len(), 32, "Blob should be exactly 32 bytes");
            assert!(d > U256::from(0), "Deadline should be positive");
            assert_eq!(d, U256::from(1735689600u64), "Deadline should match");
        }
        _ => panic!("Should be Withdraw variant"),
    }

    Ok(())
}

#[test]
fn test_signing_data_execute_variant() -> Result<()> {
    // Test SigningData::Execute variant
    let blob = FixedBytes::<32>::from([0u8; 32]);
    let deadline = U256::from(1735689600u64);
    let to_address = "0x1234567890123456789012345678901234567890".parse::<Address>()?;
    let amount = U256::from(1000000000000000000u64); // 1 ETH in wei
    let data = Some(Bytes::from(vec![0x12, 0x34, 0xab, 0xcd]));
    let expected_data = data.clone();

    let signing_data = SigningData::Execute {
        blob,
        deadline,
        to: to_address,
        amount,
        data,
    };

    match signing_data {
        SigningData::Execute {
            blob: b,
            deadline: d,
            to: t,
            amount: a,
            data: data_bytes,
        } => {
            assert_eq!(b.len(), 32, "Blob should be exactly 32 bytes");
            assert!(d > U256::from(0), "Deadline should be positive");
            assert_eq!(t, to_address, "Destination address should match");
            assert_eq!(a, amount, "Amount should match");
            assert_eq!(data_bytes, expected_data, "Data should match");
        }
        _ => panic!("Should be Execute variant"),
    }

    Ok(())
}

#[test]
fn test_wallet_status_validation() -> Result<()> {
    // Test valid WalletStatus
    let address = "0xd7baae85d719c2e8e27a70194471ef4b6b253d33".parse::<Address>()?;
    let gpg_verifier = "0x1234567890123456789012345678901234567890".parse::<Address>()?;
    let balance = U256::from(1000000000000000000u64); // 1 ETH in wei
    let nonce = U256::from(5u64);
    let transaction_count = U256::from(10u64);

    let status = WalletStatus {
        address,
        is_deployed: true,
        balance,
        nonce,
        gpg_verifier,
        transaction_count,
    };

    assert_eq!(status.address, address, "Address should match");
    assert_eq!(status.is_deployed, true, "Deployment status should match");
    assert_eq!(status.balance, balance, "Balance should match");
    assert_eq!(status.nonce, nonce, "Nonce should match");
    assert_eq!(
        status.gpg_verifier, gpg_verifier,
        "GPG verifier should match"
    );
    assert_eq!(
        status.transaction_count, transaction_count,
        "Transaction count should match"
    );

    // Validate address format
    assert_eq!(
        status.address.to_string().len(),
        42,
        "Address should be 42 characters"
    );
    assert!(
        status.address.to_string().starts_with("0x"),
        "Address should start with 0x"
    );
    assert_eq!(
        status.gpg_verifier.to_string().len(),
        42,
        "GPG verifier should be 42 characters"
    );

    // Validate non-negative values
    assert!(
        status.balance >= U256::from(0),
        "Balance should be non-negative"
    );
    assert!(
        status.nonce >= U256::from(0),
        "Nonce should be non-negative"
    );
    assert!(
        status.transaction_count >= U256::from(0),
        "Transaction count should be non-negative"
    );

    Ok(())
}

#[test]
fn test_signing_result_validation() -> Result<()> {
    // Test valid SigningResult
    let signature = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
    let public_key = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

    let result = SigningResult {
        signature: signature.to_string(),
        public_key: public_key.to_string(),
    };

    assert_eq!(result.signature, signature, "Signature should match");
    assert_eq!(result.public_key, public_key, "Public key should match");

    // Validate hex format (basic check)
    assert!(
        result.signature.chars().all(|c| c.is_ascii_hexdigit()),
        "Signature should be hex"
    );
    assert!(
        result.public_key.chars().all(|c| c.is_ascii_hexdigit()),
        "Public key should be hex"
    );

    Ok(())
}

#[test]
fn test_data_model_serialization() -> Result<()> {
    // Test JSON serialization/deserialization

    // Test AddressPrediction serialization
    let address = "0xd7baae85d719c2e8e27a70194471ef4b6b253d33".parse::<Address>()?;
    let prediction = AddressPrediction {
        wallet_address: address,
        is_deployed: true,
    };

    let json = serde_json::to_string(&prediction)?;
    let deserialized: AddressPrediction = serde_json::from_str(&json)?;

    assert_eq!(
        prediction, deserialized,
        "Serialization should preserve data"
    );

    // Test SigningData serialization
    let blob = FixedBytes::<32>::from([0u8; 32]);
    let deadline = U256::from(1735689600u64);
    let signing_data = SigningData::Withdraw { blob, deadline };

    let json = serde_json::to_string(&signing_data)?;
    let deserialized: SigningData = serde_json::from_str(&json)?;

    assert_eq!(
        signing_data, deserialized,
        "Serialization should preserve data"
    );

    Ok(())
}
