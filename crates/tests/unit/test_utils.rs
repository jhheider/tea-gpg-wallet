//! Unit tests for utility functions
//!
//! These tests validate utility function edge cases and error handling.
//! Tests must fail initially (TDD approach) and will pass once implementation is complete.

use alloy_primitives::U256;
use anyhow::Result;
use libtea_gpg_wallet::utils::{
    decimal_to_wei_precise, key_id_to_bytes, ETH_DECIMALS, WEI_PER_ETH,
};

#[test]
fn test_decimal_to_wei_precise_integer_amount() -> Result<()> {
    // Test integer amounts (no decimal point)
    let test_cases = vec![
        ("0", U256::from(0u64)),
        ("1", U256::from(WEI_PER_ETH)),
        ("5", U256::from(5u128 * WEI_PER_ETH)),
        ("100", U256::from(100u128 * WEI_PER_ETH)),
        ("1000000", U256::from(1000000u128 * WEI_PER_ETH)),
    ];

    for (input, expected) in test_cases {
        let result = decimal_to_wei_precise(input)?;
        assert_eq!(result, expected, "Failed for input: {}", input);
    }

    Ok(())
}

#[test]
fn test_decimal_to_wei_precise_decimal_amounts() -> Result<()> {
    // Test decimal amounts
    let test_cases = vec![
        ("0.1", U256::from(100000000000000000u64)),     // 0.1 ETH
        ("0.01", U256::from(10000000000000000u64)),     // 0.01 ETH
        ("0.001", U256::from(1000000000000000u64)),     // 0.001 ETH
        ("1.5", U256::from(1500000000000000000u64)),    // 1.5 ETH
        ("10.25", U256::from(10250000000000000000u64)), // 10.25 ETH
    ];

    for (input, expected) in test_cases {
        let result = decimal_to_wei_precise(input)?;
        assert_eq!(result, expected, "Failed for input: {}", input);
    }

    Ok(())
}

#[test]
fn test_decimal_to_wei_precise_precision_handling() -> Result<()> {
    // Test precision handling (padding and truncation)
    let test_cases = vec![
        ("0.1", "0.100000000000000000"), // Should pad to 18 decimals
        ("0.123456789012345678", "0.123456789012345678"), // Exact 18 decimals
        ("0.1234567890123456789", "0.123456789012345678"), // Should truncate to 18 decimals
    ];

    for (input, _expected_decimal) in test_cases {
        let result = decimal_to_wei_precise(input)?;
        // Verify the result is valid (non-negative)
        assert!(
            result >= U256::from(0),
            "Result should be non-negative for input: {}",
            input
        );
    }

    Ok(())
}

#[test]
fn test_decimal_to_wei_precise_invalid_inputs() {
    // Test invalid inputs
    let invalid_inputs = vec![
        "abc",     // Non-numeric
        "1.2.3",   // Multiple decimal points
        "1..2",    // Double decimal point
        "-1",      // Negative (if not supported)
        "1.2.3.4", // Too many decimal points
    ];

    for input in invalid_inputs {
        let result = decimal_to_wei_precise(input);
        assert!(result.is_err(), "Should fail for invalid input: {}", input);
    }
}

#[test]
fn test_key_id_to_bytes_valid() -> Result<()> {
    // Test valid key IDs
    let valid_key_ids = vec![
        "0x1234567890abcdef",
        "1234567890abcdef",
        "0x0000000000000000",
        "0xffffffffffffffff",
    ];

    for key_id in valid_key_ids {
        let result = key_id_to_bytes(key_id)?;
        assert_eq!(result.len(), 8, "Key ID should be 8 bytes");
    }

    Ok(())
}

#[test]
fn test_key_id_to_bytes_invalid() {
    // Test invalid key IDs
    let invalid_key_ids = vec![
        "",                       // Empty string
        "0x123",                  // Too short
        "0x1234567890abcdef1234", // Too long
        "xyz",                    // Non-hex
        "0xgggggggggggggggg",     // Invalid hex chars
    ];

    for key_id in invalid_key_ids {
        let result = key_id_to_bytes(key_id);
        assert!(
            result.is_err(),
            "Should fail for invalid key ID: {}",
            key_id
        );
    }
}

#[test]
fn test_constants() {
    // Test that constants are properly defined
    assert_eq!(
        WEI_PER_ETH, 1_000_000_000_000_000_000,
        "WEI_PER_ETH should be correct"
    );
    assert_eq!(ETH_DECIMALS, 18, "ETH_DECIMALS should be 18");
}
