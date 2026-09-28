//! Integration tests for complete workflow
//!
//! These tests validate the complete user workflow from quickstart.md.
//! Tests must fail initially (TDD approach) and will pass once implementation is complete.

use std::process::Command;

use anyhow::Result;

/// Test helper to run the CLI tool with arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "tea-gpg-wallet", "--"]);
    cmd.args(args);
    cmd.output().expect("Failed to execute CLI")
}

/// Test helper to run CLI with environment variables
fn run_cli_with_env(args: &[&str], env_vars: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "tea-gpg-wallet", "--"]);
    cmd.args(args);

    for (key, value) in env_vars {
        cmd.env(key, value);
    }

    cmd.output().expect("Failed to execute CLI")
}

#[tokio::test]
async fn test_complete_workflow_setup_and_configuration() -> Result<()> {
    // Test 1: Setup and configuration check
    let output = run_cli(&["config"]);

    // Should succeed and show configuration
    assert!(
        output.status.success(),
        "Configuration check should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RPC URL:"), "Should show RPC URL");
    assert!(
        stdout.contains("Deployer address:"),
        "Should show deployer address"
    );
    assert!(stdout.contains("Network:"), "Should show network info");

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_wallet_address_prediction() -> Result<()> {
    // Test 2: Wallet address prediction
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);

    // Should succeed and show predicted address
    assert!(output.status.success(), "Address prediction should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Predicted address for key ID"),
        "Should show predicted address"
    );
    assert!(
        stdout.contains("95469C7E3DFC90B1"),
        "Should show the key ID"
    );
    assert!(
        stdout.contains("deployed") || stdout.contains("not deployed"),
        "Should show deployment status"
    );

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_wallet_deployment() -> Result<()> {
    // Test 3: Wallet deployment (may fail due to network/private key issues)
    let output = run_cli_with_env(
        &["deploy", "95469C7E3DFC90B1"],
        &[(
            "PRIVATE_KEY",
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )],
    );

    // May fail due to network issues, but should show proper error handling
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("Invalid")
                || stderr.contains("Network"),
            "Should provide meaningful error message for deployment failure"
        );
    } else {
        // If it succeeds, should show deployment confirmation
        assert!(
            stdout.contains("Deployed address") || stdout.contains("Wallet deployed"),
            "Should show deployment confirmation"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_tea_sending() -> Result<()> {
    // Test 4: Send initial TEA amount (may fail due to network/balance issues)
    let output = run_cli_with_env(
        &["send", "95469C7E3DFC90B1", "1.5"],
        &[(
            "PRIVATE_KEY",
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )],
    );

    // May fail due to network or balance issues, but should show proper handling
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("balance")
                || stderr.contains("network"),
            "Should provide meaningful error message for send failure"
        );
    } else {
        // If it succeeds, should show sending confirmation
        assert!(
            stdout.contains("Sending") || stdout.contains("sent successfully"),
            "Should show sending confirmation"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_wallet_status_verification() -> Result<()> {
    // Test 5: Verify wallet state
    let output = run_cli(&["status", "95469C7E3DFC90B1"]);

    // Should succeed and show comprehensive status
    assert!(output.status.success(), "Status check should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Address:"), "Should show address");
    assert!(stdout.contains("Status:"), "Should show status");
    assert!(stdout.contains("Balance:"), "Should show balance");
    assert!(stdout.contains("Nonce:"), "Should show nonce");
    assert!(stdout.contains("GPG Verifier:"), "Should show GPG verifier");
    assert!(
        stdout.contains("Transaction Count:"),
        "Should show transaction count"
    );

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_gpg_signed_transaction() -> Result<()> {
    // Test 6: Perform GPG-signed transaction (may fail due to wallet not deployed or signature
    // issues)
    let output = run_cli(&[
        "execute",
        "95469C7E3DFC90B1",
        "0x1234567890123456789012345678901234567890",
        "0.5",
    ]);

    // May fail due to wallet not deployed, signature issues, or other problems
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("deployed")
                || stderr.contains("signature")
                || stderr.contains("authentication"),
            "Should provide meaningful error message for execute failure"
        );
    } else {
        // If it succeeds, should show execution confirmation
        assert!(
            stdout.contains("Executing") || stdout.contains("executed successfully"),
            "Should show execution confirmation"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_fund_sweeping() -> Result<()> {
    // Test 7: Transfer all funds (may fail due to wallet not deployed or zero balance)
    let output = run_cli(&[
        "sweep",
        "95469C7E3DFC90B1",
        "0x1234567890123456789012345678901234567890",
    ]);

    // May fail due to wallet not deployed or other issues
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("deployed")
                || stderr.contains("balance"),
            "Should provide meaningful error message for sweep failure"
        );
    } else {
        // If it succeeds or has zero balance, should handle gracefully
        assert!(
            stdout.contains("Sweeping")
                || stdout.contains("balance")
                || stdout.contains("zero")
                || stdout.contains("No balance"),
            "Should handle sweep operation gracefully"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_final_state_verification() -> Result<()> {
    // Test 8: Confirm final state
    let output = run_cli(&["status", "95469C7E3DFC90B1"]);

    // Should succeed and show final wallet state
    assert!(output.status.success(), "Final status check should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Address:"), "Should show address");
    assert!(stdout.contains("Status:"), "Should show status");
    assert!(stdout.contains("Balance:"), "Should show balance");
    assert!(stdout.contains("Nonce:"), "Should show nonce");
    assert!(
        stdout.contains("Transaction Count:"),
        "Should show transaction count"
    );

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_error_handling() -> Result<()> {
    // Test error handling scenarios

    // Test invalid key ID
    let output = run_cli(&["find", "invalid-key"]);
    assert!(!output.status.success(), "Should fail with invalid key ID");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"), "Should start with 'Error:'");

    // Test missing private key for deployment
    let output = run_cli(&["deploy", "95469C7E3DFC90B1"]);
    assert!(!output.status.success(), "Should fail without private key");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("PRIVATE_KEY") || stderr.contains("private key"),
        "Should mention PRIVATE_KEY requirement"
    );

    // Test invalid amount for send
    let output = run_cli_with_env(
        &["send", "95469C7E3DFC90B1", "invalid-amount"],
        &[(
            "PRIVATE_KEY",
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )],
    );
    assert!(!output.status.success(), "Should fail with invalid amount");

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_authentication_methods() -> Result<()> {
    // Test different authentication methods

    // Test direct key ID (should work)
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);
    assert!(output.status.success(), "Direct key ID should work");

    // Test BPB integration (may fail if BPB not available)
    let output = run_cli(&["find", "--bpb"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        // Should provide helpful error message for BPB issues
        assert!(
            stderr.contains("bpb")
                || stderr.contains("BPB")
                || stderr.contains("installation")
                || stderr.contains("Error:"),
            "Should provide helpful error message for BPB issues"
        );
    }

    // Test GPG email lookup (may fail if GPG not available or email not found)
    let output = run_cli(&["find", "--gpg", "test@example.com"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        // Should provide helpful error message for GPG issues
        assert!(
            stderr.contains("gpg")
                || stderr.contains("GPG")
                || stderr.contains("email")
                || stderr.contains("Error:"),
            "Should provide helpful error message for GPG issues"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_complete_workflow_performance_requirements() -> Result<()> {
    // Test performance requirements (startup time, operation times)

    // Test CLI startup time (should be ≤ 1 second)
    let start = std::time::Instant::now();
    let output = run_cli(&["--help"]);
    let startup_duration = start.elapsed();

    assert!(output.status.success(), "Help command should succeed");
    assert!(
        startup_duration.as_millis() <= 1000,
        "CLI startup should be ≤ 1 second"
    );

    // Test query operation time (should be ≤ 5 seconds)
    let start = std::time::Instant::now();
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);
    let query_duration = start.elapsed();

    assert!(output.status.success(), "Find command should succeed");
    assert!(
        query_duration.as_millis() <= 5000,
        "Query operations should be ≤ 5 seconds"
    );

    Ok(())
}
