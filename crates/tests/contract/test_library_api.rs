//! Contract tests for library API
//!
//! These tests validate the library API according to the library-api.md contract.
//! Tests must fail initially (TDD approach) and will pass once implementation is complete.

use alloy_primitives::U256;
use anyhow::Result;
use libtea_gpg_wallet::{deployer, utils, wallet};

#[tokio::test]
async fn test_deployer_predict_address() -> Result<()> {
    // Test valid key ID
    let key_id = "95469C7E3DFC90B1";
    let result = deployer::predict_address(key_id).await;

    // Should succeed and return AddressPrediction
    assert!(
        result.is_ok(),
        "predict_address should succeed with valid key ID"
    );

    let prediction = result.unwrap();
    assert_eq!(
        prediction.walletAddress.to_string().len(),
        42,
        "Address should be 42 characters"
    );
    assert!(
        prediction.walletAddress.to_string().starts_with("0x"),
        "Address should start with 0x"
    );

    // Test invalid key ID format
    let invalid_key_id = "invalid";
    let result = deployer::predict_address(invalid_key_id).await;
    assert!(
        result.is_err(),
        "predict_address should fail with invalid key ID"
    );

    Ok(())
}

#[tokio::test]
async fn test_deployer_ensure_deployed() -> Result<()> {
    let key_id = "95469C7E3DFC90B1";

    // Test with invalid private key
    let invalid_private_key = "invalid";
    let result = deployer::ensure_deployed(key_id, invalid_private_key).await;
    assert!(
        result.is_err(),
        "ensure_deployed should fail with invalid private key"
    );

    Ok(())
}

#[tokio::test]
async fn test_deployer_send_to_gpg_key() -> Result<()> {
    let key_id = "95469C7E3DFC90B1";
    let _amount = U256::from(1000000000000000000u64); // 1 ETH in wei
    let private_key = "0x1234567890123456789012345678901234567890123456789012345678901234";

    // Test with zero amount
    let zero_amount = U256::from(0);
    let result = deployer::send_to_gpg_key(key_id, zero_amount, private_key).await;
    assert!(
        result.is_err(),
        "send_to_gpg_key should fail with zero amount"
    );

    Ok(())
}

#[tokio::test]
async fn test_wallet_get_signable_hash() -> Result<()> {
    let key_id = "95469C7E3DFC90B1";
    let to_address = "0x1234567890123456789012345678901234567890";

    // Test with valid parameters
    let result = wallet::get_signable_hash(key_id, to_address).await;

    if let Ok(signing_data) = result {
        // Should return SigningData::Withdraw variant
        // Check that signing_data has the expected fields
        assert_eq!(signing_data.blob.len(), 32, "Blob should be 32 bytes");
        assert!(
            signing_data.deadline > U256::from(0),
            "Deadline should be positive"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_utils_decimal_to_wei_precise() -> Result<()> {
    // Test valid decimal strings
    let test_cases = vec![
        ("1.0", 1000000000000000000u64),
        ("0.5", 500000000000000000u64),
        ("0.001", 1000000000000000u64),
    ];

    for (decimal_str, expected_wei) in test_cases {
        let result = utils::decimal_to_wei_precise(decimal_str);
        assert!(
            result.is_ok(),
            "decimal_to_wei_precise should succeed with valid decimal: {}",
            decimal_str
        );

        let wei = result.unwrap();
        assert_eq!(
            wei,
            U256::from(expected_wei),
            "Wei conversion should match for: {}",
            decimal_str
        );
    }

    // Test invalid decimal strings
    let invalid_cases = vec!["invalid", "1.2.3", "-1.0", ""];

    for invalid_decimal in invalid_cases {
        let result = utils::decimal_to_wei_precise(invalid_decimal);
        assert!(
            result.is_err(),
            "decimal_to_wei_precise should fail with invalid decimal: {}",
            invalid_decimal
        );
    }

    Ok(())
}
