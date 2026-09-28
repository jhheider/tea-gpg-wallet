# Data Model: TEA GPG Wallet CLI Tool

**Date**: 2024-12-19  
**Feature**: TEA GPG Wallet CLI Tool  
**Purpose**: Define core data structures and their relationships

## Core Entities

### AddressPrediction
Represents a predicted or deployed wallet address for a GPG key.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AddressPrediction {
    pub wallet_address: Address,    // Ethereum address (20 bytes)
    pub is_deployed: bool,          // Whether contract is deployed
}
```

**Validation Rules**:
- `wallet_address`: Must be valid Ethereum address (checksum format)
- `is_deployed`: Boolean indicating deployment status

**State Transitions**:
- `is_deployed: false → true`: When wallet contract is deployed
- `is_deployed: true → true`: No change (deployed wallets remain deployed)

**Usage**: Returned by prediction and deployment operations

### SigningData
Represents data to be signed for different wallet operations.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SigningData {
    Withdraw {
        blob: FixedBytes<32>,       // EIP-712 hash blob
        deadline: U256,             // Transaction deadline
    },
    Execute {
        blob: FixedBytes<32>,       // EIP-712 hash blob
        deadline: U256,             // Transaction deadline
        to: Address,                // Destination address
        amount: U256,               // Amount to transfer
        data: Option<Bytes>,        // Optional contract call data (max 1KB)
    },
}
```

**Validation Rules**:
- `blob`: Must be exactly 32 bytes (EIP-712 hash)
- `deadline`: Must be future timestamp (Unix epoch seconds)
- `to`: Must be valid Ethereum address (checksum format)
- `amount`: Must be positive (greater than 0)
- `data`: If present, must be valid hex string with maximum 1KB length

**State Transitions**:
- Created with future deadline → Used for signing → Expires after deadline

**Usage**: Passed to GPG/BPB for signing operations

### WalletStatus
Represents complete wallet state and configuration information.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct WalletStatus {
    pub address: Address,           // Wallet contract address
    pub is_deployed: bool,          // Deployment status
    pub balance: U256,              // Current TEA balance (wei)
    pub nonce: U256,                // Next transaction nonce
    pub gpg_verifier: Address,      // GPG verifier contract address
    pub transaction_count: U256,    // Total transaction count
}
```

**Validation Rules**:
- `address`: Must be valid Ethereum address (checksum format)
- `is_deployed`: Boolean indicating deployment status
- `balance`: Must be non-negative (0 or positive)
- `nonce`: Must be non-negative (0 or positive)
- `gpg_verifier`: Must be valid Ethereum address (checksum format)
- `transaction_count`: Must be non-negative (0 or positive)

**State Transitions**:
- `balance`: Increases on incoming transfers, decreases on outgoing transfers
- `nonce`: Increments with each transaction
- `transaction_count`: Increments with each transaction
- `is_deployed`: Changes from false to true on deployment (irreversible)

**Usage**: Returned by status command and wallet queries

### SigningResult
Represents the result of a GPG/BPB signing operation.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct SigningResult {
    pub signature: String,          // Hex-encoded signature
    pub public_key: String,         // Hex-encoded public key
}
```

**Validation Rules**:
- `signature`: Must be valid hex string (no 0x prefix)
- `public_key`: Must be valid hex string (no 0x prefix)

**State Transitions**:
- Created after successful signing → Used for transaction submission → Discarded after use

**Usage**: Returned by GPG/BPB integration, used for transaction execution

## Supporting Types

### WalletError
Comprehensive error types for all wallet operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Invalid key ID: {0}")]
    InvalidKeyId(String),
    
    #[error("Wallet not deployed for key ID: {0}")]
    WalletNotDeployed(String),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: U256, available: U256 },
    
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid contract call data: {0}")]
    InvalidContractData(String),
    
    #[error("Deadline expired: {deadline}")]
    DeadlineExpired { deadline: U256 },
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}
```

**Validation Rules**:
- All error messages must be user-friendly and actionable
- Error context must not leak sensitive information
- Error codes must be consistent across operations

## Data Flow Patterns

### Wallet Deployment Flow
1. `AddressPrediction` created with `is_deployed: false`
2. Deployment transaction submitted
3. `AddressPrediction` updated with `is_deployed: true`

### Transaction Execution Flow
1. `SigningData` created with operation parameters
2. `SigningData` passed to GPG/BPB for signing
3. `SigningResult` returned with signature and public key
4. Transaction submitted with signature
5. `WalletStatus` updated with new balance/nonce

### Status Query Flow
1. GPG key ID provided
2. Wallet address predicted/retrieved
3. Blockchain state queried
4. `WalletStatus` constructed and returned

## Validation Rules Summary

### GPG Key ID
- Format: 16-character hexadecimal string
- Example: `95469C7E3DFC90B1`
- Validation: Must be valid hex, exactly 16 characters

### Ethereum Addresses
- Format: Checksum format (EIP-55)
- Example: `0xd7baae85d719c2e8e27a70194471ef4b6b253d33`
- Validation: Must be valid 20-byte address with proper checksum

### TEA Amounts
- Format: Decimal string or U256
- Example: `"1.5"`, `"0.001"`, `1000000000000000000`
- Validation: Must be positive, within precision limits

### Contract Call Data
- Format: Hex string (with or without 0x prefix)
- Example: `"0x1234abcd"`, `"1234abcd"`
- Validation: Must be valid hex, maximum 1KB length

### Transaction Deadlines
- Format: Unix epoch timestamp (seconds)
- Example: `1735689600` (2025-01-01 00:00:00 UTC)
- Validation: Must be future timestamp

## Serialization Requirements

### JSON Serialization
- All data types must implement `Serialize` and `Deserialize`
- Use snake_case for field names
- Include version information for future compatibility

### CLI Output Format
- Human-readable format for user-facing output
- Machine-readable format for automation (--json flag)
- Consistent formatting across all commands

---

*Data model designed to support all functional requirements with strong type safety and validation.*