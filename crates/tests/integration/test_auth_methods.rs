//! Integration tests for authentication methods
//!
//! These tests validate all authentication methods (direct key, BPB, GPG email).
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

#[tokio::test]
async fn test_direct_gpg_key_id_usage() -> Result<()> {
    // Test direct GPG key ID usage
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);

    // Should succeed with direct key ID
    assert!(output.status.success(), "Direct key ID should work");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Predicted address for key ID"),
        "Should show predicted address"
    );
    assert!(
        stdout.contains("95469C7E3DFC90B1"),
        "Should show the key ID"
    );

    Ok(())
}

#[tokio::test]
async fn test_direct_gpg_key_id_validation() -> Result<()> {
    // Test invalid key ID format
    let output = run_cli(&["find", "invalid-key"]);

    // Should fail with invalid key ID
    assert!(!output.status.success(), "Should fail with invalid key ID");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"), "Should start with 'Error:'");

    // Test wrong length key ID
    let output = run_cli(&["find", "95469C7E3DFC90B"]); // 15 chars instead of 16
    assert!(
        !output.status.success(),
        "Should fail with wrong length key ID"
    );

    Ok(())
}

#[tokio::test]
async fn test_bpb_integration() -> Result<()> {
    // Test BPB integration
    let output = run_cli(&["find", "--bpb"]);

    // May fail if BPB is not available, but should handle gracefully
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide helpful error message for BPB issues
        assert!(
            stderr.contains("bpb")
                || stderr.contains("BPB")
                || stderr.contains("installation")
                || stderr.contains("Error:")
                || stderr.contains("not found")
                || stderr.contains("command"),
            "Should provide helpful error message for BPB issues"
        );
    } else {
        // If it succeeds, should show predicted address
        assert!(
            stdout.contains("Predicted address"),
            "Should show predicted address on success"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_bpb_integration_with_deploy() -> Result<()> {
    // Test BPB integration with deploy command
    let output = run_cli(&["deploy", "--bpb"]);

    // Should fail without private key
    assert!(!output.status.success(), "Should fail without private key");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("PRIVATE_KEY")
            || stderr.contains("private key")
            || stderr.contains("bpb")
            || stderr.contains("BPB")
            || stderr.contains("Error:"),
        "Should mention PRIVATE_KEY requirement or BPB issues"
    );

    Ok(())
}

#[tokio::test]
async fn test_gpg_email_lookup() -> Result<()> {
    // Test GPG email lookup
    let output = run_cli(&["find", "--gpg", "test@example.com"]);

    // May fail if GPG is not available or email not found
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide helpful error message for GPG issues
        assert!(
            stderr.contains("gpg")
                || stderr.contains("GPG")
                || stderr.contains("email")
                || stderr.contains("Error:")
                || stderr.contains("not found")
                || stderr.contains("command")
                || stderr.contains("key")
                || stderr.contains("lookup"),
            "Should provide helpful error message for GPG issues"
        );
    } else {
        // If it succeeds, should show predicted address
        assert!(
            stdout.contains("Predicted address"),
            "Should show predicted address on success"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_gpg_email_lookup_validation() -> Result<()> {
    // Test invalid email format
    let output = run_cli(&["find", "--gpg", "invalid-email"]);

    // Should fail with invalid email format
    assert!(
        !output.status.success(),
        "Should fail with invalid email format"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("email")
            || stderr.contains("Email")
            || stderr.contains("gpg")
            || stderr.contains("GPG")
            || stderr.contains("Error:"),
        "Should mention email or GPG issues"
    );

    // Test missing email parameter
    let output = run_cli(&["find", "--gpg"]);

    // Should fail with missing email parameter
    assert!(
        !output.status.success(),
        "Should fail with missing email parameter"
    );

    Ok(())
}

#[tokio::test]
async fn test_authentication_method_fallback() -> Result<()> {
    // Test that the tool gracefully handles missing authentication tools

    // Test BPB fallback when not available
    let output = run_cli(&["find", "--bpb"]);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should provide helpful error message
        assert!(
            stderr.contains("bpb")
                || stderr.contains("BPB")
                || stderr.contains("installation")
                || stderr.contains("Error:")
                || stderr.contains("not found")
                || stderr.contains("command"),
            "Should provide helpful error message for missing BPB"
        );
    }

    // Test GPG fallback when not available
    let output = run_cli(&["find", "--gpg", "test@example.com"]);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should provide helpful error message
        assert!(
            stderr.contains("gpg")
                || stderr.contains("GPG")
                || stderr.contains("installation")
                || stderr.contains("Error:")
                || stderr.contains("not found")
                || stderr.contains("command"),
            "Should provide helpful error message for missing GPG"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_authentication_method_consistency() -> Result<()> {
    // Test that all authentication methods work consistently across commands

    let test_key_id = "95469C7E3DFC90B1";

    // Test find command with direct key ID
    let output = run_cli(&["find", test_key_id]);
    assert!(
        output.status.success(),
        "Find with direct key ID should work"
    );

    // Test deploy command with direct key ID (should fail without private key)
    let output = run_cli(&["deploy", test_key_id]);
    assert!(
        !output.status.success(),
        "Deploy should fail without private key"
    );

    // Test status command with direct key ID
    let output = run_cli(&["status", test_key_id]);
    assert!(
        output.status.success(),
        "Status with direct key ID should work"
    );

    Ok(())
}

#[tokio::test]
async fn test_authentication_error_handling() -> Result<()> {
    // Test error handling for authentication failures

    // Test missing authentication method
    let output = run_cli(&["find"]);
    assert!(
        !output.status.success(),
        "Should fail without authentication method"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required")
            || stderr.contains("Required")
            || stderr.contains("key")
            || stderr.contains("Key")
            || stderr.contains("Error:"),
        "Should mention required authentication method"
    );

    // Test conflicting authentication methods
    let output = run_cli(&["find", "95469C7E3DFC90B1", "--bpb"]);
    // This should either succeed with the direct key ID or provide a clear error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        // Should provide clear error message about conflicting methods
        assert!(
            stderr.contains("conflict")
                || stderr.contains("Conflict")
                || stderr.contains("multiple")
                || stderr.contains("Multiple")
                || stderr.contains("Error:"),
            "Should mention conflicting authentication methods"
        );
    } else {
        // If it succeeds, should use one of the methods
        assert!(
            stdout.contains("Predicted address"),
            "Should show predicted address"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_authentication_method_performance() -> Result<()> {
    // Test performance of different authentication methods

    // Test direct key ID performance (should be fastest)
    let start = std::time::Instant::now();
    let output = run_cli(&["find", "95469C7E3DFC90B1"]);
    let direct_duration = start.elapsed();

    assert!(output.status.success(), "Direct key ID should work");
    assert!(
        direct_duration.as_millis() <= 5000,
        "Direct key ID should be fast (≤ 5 seconds)"
    );

    // Test BPB performance (may be slower due to subprocess)
    let start = std::time::Instant::now();
    let output = run_cli(&["find", "--bpb"]);
    let bpb_duration = start.elapsed();

    // Should complete within reasonable time even if it fails
    assert!(
        bpb_duration.as_millis() <= 10000,
        "BPB should complete within 10 seconds"
    );

    // Test GPG performance (may be slower due to subprocess)
    let start = std::time::Instant::now();
    let output = run_cli(&["find", "--gpg", "test@example.com"]);
    let gpg_duration = start.elapsed();

    // Should complete within reasonable time even if it fails
    assert!(
        gpg_duration.as_millis() <= 10000,
        "GPG should complete within 10 seconds"
    );

    Ok(())
}
