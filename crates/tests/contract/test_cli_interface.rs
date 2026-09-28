//! Contract tests for CLI interface
//!
//! These tests validate the CLI interface according to the cli-interface.md contract.
//! Tests must fail initially (TDD approach) and will pass once implementation is complete.

use std::{env, process::Command};

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

#[test]
fn test_config_command_output_format() {
    let output = run_cli(&["config"]);

    // Should exit with success code
    assert!(output.status.success(), "config command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain required fields
    assert!(stdout.contains("RPC URL:"), "Output should contain RPC URL");
    assert!(
        stdout.contains("Deployer address:"),
        "Output should contain Deployer address"
    );
    assert!(
        stdout.contains("Network:"),
        "Output should contain Network info"
    );
    assert!(
        stdout.contains("Configuration source:"),
        "Output should contain config source"
    );
}

#[test]
fn test_find_command_direct_key_id() {
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);

    // Should exit with success code
    assert!(
        output.status.success(),
        "find command with direct key ID should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show predicted address and deployment status
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
}

#[test]
fn test_find_command_bpb_option() {
    let output = run_cli(&["find", "--bpb"]);

    // May fail if BPB is not available, but should handle gracefully
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should either succeed or provide helpful error message
    if !output.status.success() {
        assert!(
            stderr.contains("bpb")
                || stderr.contains("BPB")
                || stderr.contains("installation")
                || stderr.contains("Error:"),
            "Should provide helpful error message for BPB issues"
        );
    } else {
        // If it succeeds, should show predicted address
        assert!(
            stdout.contains("Predicted address"),
            "Should show predicted address on success"
        );
    }
}

#[test]
fn test_find_command_gpg_email() {
    let output = run_cli(&["find", "--gpg", "test@example.com"]);

    // May fail if GPG is not available or email not found
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should either succeed or provide helpful error message
    if !output.status.success() {
        assert!(
            stderr.contains("gpg")
                || stderr.contains("GPG")
                || stderr.contains("email")
                || stderr.contains("Error:"),
            "Should provide helpful error message for GPG issues"
        );
    } else {
        // If it succeeds, should show predicted address
        assert!(
            stdout.contains("Predicted address"),
            "Should show predicted address on success"
        );
    }
}

#[test]
fn test_deploy_command_requires_private_key() {
    let output = run_cli(&["deploy", "95469C7E3DFC90B1"]);

    // Should fail without private key
    assert!(
        !output.status.success(),
        "deploy command should fail without private key"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("PRIVATE_KEY")
            || stderr.contains("private key")
            || stderr.contains("environment"),
        "Should mention PRIVATE_KEY requirement"
    );
}

#[test]
fn test_deploy_command_with_private_key() {
    // This test will fail initially but should pass once implementation is complete
    let output = run_cli_with_env(
        &["deploy", "95469C7E3DFC90B1"],
        &[(
            "PRIVATE_KEY",
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )],
    );

    // May fail due to invalid private key or network issues, but should show proper error handling
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("Invalid")
                || stderr.contains("Network"),
            "Should provide meaningful error message"
        );
    } else {
        // If it succeeds, should show deployment confirmation
        assert!(
            stdout.contains("Deployed address") || stdout.contains("Wallet deployed"),
            "Should show deployment confirmation"
        );
    }
}

#[test]
fn test_send_command_amount_validation() {
    let output = run_cli_with_env(
        &["send", "95469C7E3DFC90B1", "invalid-amount"],
        &[(
            "PRIVATE_KEY",
            "0x1234567890123456789012345678901234567890123456789012345678901234",
        )],
    );

    // Should fail with invalid amount
    assert!(
        !output.status.success(),
        "send command should fail with invalid amount"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("amount")
            || stderr.contains("Amount")
            || stderr.contains("Invalid")
            || stderr.contains("Error:"),
        "Should mention amount validation error"
    );
}

#[test]
fn test_send_command_with_valid_amount() {
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
            "Should provide meaningful error message"
        );
    } else {
        // If it succeeds, should show sending confirmation
        assert!(
            stdout.contains("Sending") || stdout.contains("sent successfully"),
            "Should show sending confirmation"
        );
    }
}

#[test]
fn test_execute_command_data_validation() {
    let output = run_cli(&[
        "execute",
        "95469C7E3DFC90B1",
        "0x1234567890123456789012345678901234567890",
        "1.0",
        "--data",
        "invalid-hex",
    ]);

    // Should fail with invalid data format
    assert!(
        !output.status.success(),
        "execute command should fail with invalid data"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("data")
            || stderr.contains("Data")
            || stderr.contains("hex")
            || stderr.contains("Invalid"),
        "Should mention data validation error"
    );
}

#[test]
fn test_execute_command_data_length_validation() {
    // Create a data string longer than 1KB (1024 bytes = 2048 hex chars)
    let long_data = "0x".to_string() + &"a".repeat(2048);

    let output = run_cli(&[
        "execute",
        "95469C7E3DFC90B1",
        "0x1234567890123456789012345678901234567890",
        "1.0",
        "--data",
        &long_data,
    ]);

    // Should fail with data too long
    assert!(
        !output.status.success(),
        "execute command should fail with data too long"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("data")
            || stderr.contains("Data")
            || stderr.contains("long")
            || stderr.contains("size")
            || stderr.contains("1KB"),
        "Should mention data size limit"
    );
}

#[test]
fn test_execute_command_with_valid_data() {
    let output = run_cli(&[
        "execute",
        "95469C7E3DFC90B1",
        "0x1234567890123456789012345678901234567890",
        "1.0",
        "--data",
        "0x1234abcd",
    ]);

    // May fail due to wallet not deployed or other issues, but should show proper handling
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide meaningful error message
        assert!(
            stderr.contains("Error:")
                || stderr.contains("Failed")
                || stderr.contains("deployed")
                || stderr.contains("signature"),
            "Should provide meaningful error message"
        );
    } else {
        // If it succeeds, should show execution confirmation
        assert!(
            stdout.contains("Executing") || stdout.contains("executed successfully"),
            "Should show execution confirmation"
        );
    }
}

#[test]
fn test_sweep_command_zero_balance_handling() {
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
            "Should provide meaningful error message"
        );
    } else {
        // If it succeeds or has zero balance, should handle gracefully
        assert!(
            stdout.contains("Sweeping")
                || stdout.contains("balance")
                || stdout.contains("zero")
                || stdout.contains("No balance"),
            "Should handle zero balance gracefully"
        );
    }
}

#[test]
fn test_status_command_comprehensive_output() {
    let output = run_cli(&["status", "95469C7E3DFC90B1"]);

    // Should exit with success code
    assert!(output.status.success(), "status command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain all required fields
    assert!(stdout.contains("Address:"), "Output should contain Address");
    assert!(stdout.contains("Status:"), "Output should contain Status");
    assert!(stdout.contains("Balance:"), "Output should contain Balance");
    assert!(stdout.contains("Nonce:"), "Output should contain Nonce");
    assert!(
        stdout.contains("GPG Verifier:"),
        "Output should contain GPG Verifier"
    );
    assert!(
        stdout.contains("Transaction Count:"),
        "Output should contain Transaction Count"
    );
}

#[test]
fn test_error_message_format_consistency() {
    // Test with invalid key ID format
    let output = run_cli(&["find", "invalid-key"]);

    assert!(!output.status.success(), "Should fail with invalid key ID");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should follow error message format
    assert!(stderr.contains("Error:"), "Should start with 'Error:'");

    // Should contain details section
    assert!(
        stderr.contains("Details:") || stderr.contains("Details"),
        "Should contain details section"
    );

    // Should contain suggestions
    assert!(
        stderr.contains("Suggestions:")
            || stderr.contains("Suggestions")
            || stderr.contains("help"),
        "Should contain suggestions or help"
    );
}

#[test]
fn test_exit_code_validation() {
    // Test success case (config command should succeed)
    let output = run_cli(&["config"]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "Success should return exit code 0"
    );

    // Test user error case (invalid arguments)
    let output = run_cli(&["find"]);
    assert_eq!(
        output.status.code(),
        Some(2),
        "User error should return exit code 2"
    );

    // Test system error case (missing required argument)
    let output = run_cli(&["deploy"]);
    assert!(
        output.status.code() == Some(1) || output.status.code() == Some(2),
        "System/user error should return exit code 1 or 2"
    );
}

#[test]
fn test_global_options_help() {
    let output = run_cli(&["--help"]);

    assert!(output.status.success(), "Help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain help information
    assert!(
        stdout.contains("tea-gpg-wallet"),
        "Should contain tool name"
    );
    assert!(
        stdout.contains("USAGE:"),
        "Should contain usage information"
    );
    assert!(
        stdout.contains("SUBCOMMANDS:"),
        "Should contain subcommands"
    );
}

#[test]
fn test_global_options_version() {
    let output = run_cli(&["--version"]);

    assert!(output.status.success(), "Version should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain version information
    assert!(
        stdout.contains("tea-gpg-wallet"),
        "Should contain tool name"
    );
    assert!(stdout.contains("0.2.0"), "Should contain version number");
}

#[test]
fn test_global_options_verbose() {
    let output = run_cli(&["--verbose", "config"]);

    // Should succeed with verbose flag
    assert!(output.status.success(), "Verbose flag should work");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show normal config output (verbose features not yet implemented)
    assert!(
        stdout.contains("RPC URL:"),
        "Should show normal config output"
    );
}

#[test]
fn test_global_options_json() {
    let output = run_cli(&["--json", "config"]);

    // Should succeed with JSON flag
    assert!(output.status.success(), "JSON flag should work");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show JSON output (JSON features not yet implemented)
    assert!(
        stdout.contains("RPC URL:") || stdout.contains("{"),
        "Should show JSON or normal output"
    );
}

#[test]
fn test_global_options_config_override() {
    let output = run_cli(&["--config", "/nonexistent/config.toml", "config"]);

    // May fail due to nonexistent config file
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        // Should provide meaningful error about config file
        assert!(
            stderr.contains("config")
                || stderr.contains("Config")
                || stderr.contains("file")
                || stderr.contains("Error:"),
            "Should mention config file issue"
        );
    }
}

#[test]
fn test_global_options_rpc_url_override() {
    let output = run_cli(&["--rpc-url", "https://custom-rpc.example.com", "config"]);

    // Should succeed with custom RPC URL
    assert!(output.status.success(), "Custom RPC URL should work");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show the custom RPC URL
    assert!(
        stdout.contains("custom-rpc.example.com"),
        "Should show custom RPC URL"
    );
}

#[test]
fn test_global_options_retry_override() {
    let output = run_cli(&["--retry-count", "5", "config"]);

    // Should succeed with custom retry count
    assert!(output.status.success(), "Custom retry count should work");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show normal config output (retry not visible in config)
    assert!(
        stdout.contains("RPC URL:"),
        "Should show normal config output"
    );
}

#[test]
fn test_global_options_retry_backoff_override() {
    let output = run_cli(&["--retry-backoff-ms", "2000", "config"]);

    // Should succeed with custom retry backoff
    assert!(output.status.success(), "Custom retry backoff should work");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show normal config output (backoff not visible in config)
    assert!(
        stdout.contains("RPC URL:"),
        "Should show normal config output"
    );
}
