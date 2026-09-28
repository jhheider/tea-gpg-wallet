# TEA GPG Wallet CLI Tool Specification

## Executive Summary

This specification defines the requirements, architecture, and implementation details for a command-line tool that enables users to interact with TEA's on-chain GPG rewards wallets. The tool provides a secure, user-friendly interface for managing TEA tokens using GPG key identities, eliminating the need for traditional private key management.

## Table of Contents

1. [Overview](#overview)
2. [Requirements](#requirements)
3. [System Architecture](#system-architecture)
4. [Command Specifications](#command-specifications)
5. [API Design](#api-design)
6. [Data Models](#data-models)
7. [Error Handling](#error-handling)
8. [Security Considerations](#security-considerations)
9. [Performance Requirements](#performance-requirements)
10. [Testing Strategy](#testing-strategy)
11. [Implementation Plan](#implementation-plan)

## Overview

### Purpose
The TEA GPG Wallet CLI tool enables users to:
- Deploy and manage GPG-based smart wallet contracts on the TEA blockchain
- Send and receive TEA tokens using GPG key authentication
- Interact with smart contracts through GPG-signed transactions
- Manage wallet state and query balances

### Key Features
- **GPG-based Authentication**: Use existing GPG keys for wallet operations
- **Multiple Key Sources**: Support direct key IDs, BPB integration, and email lookup
- **Automatic Deployment**: Deploy wallets on-demand when needed
- **Secure Signing**: Support both GPG and BPB (secure enclave) signing methods
- **Flexible Transactions**: Send arbitrary amounts and execute smart contract calls

## Requirements

### Functional Requirements

#### FR1: Wallet Management
- **FR1.1**: Predict wallet addresses for any GPG key ID
- **FR1.2**: Deploy wallet contracts on-demand
- **FR1.3**: Check wallet deployment status and balance
- **FR1.4**: Query wallet state and configuration

#### FR2: Transaction Operations
- **FR2.1**: Send TEA tokens to GPG wallets (deploy if needed)
- **FR2.2**: Send arbitrary amounts from GPG wallets using signatures
- **FR2.3**: Sweep all funds from a GPG wallet
- **FR2.4**: Execute simple transfers with optional contract call data
- **FR2.5**: Estimate gas limit, gas price, and total cost in TEA

#### FR3: Authentication Methods
- **FR3.1**: Support direct GPG key ID specification
- **FR3.2**: Integrate with BPB for secure enclave operations
- **FR3.3**: Lookup GPG keys by email address
- **FR3.4**: Support both GPG and BPB signing methods

#### FR4: User Experience
- **FR4.1**: Provide clear, colored command-line interface
- **FR4.2**: Show progress indicators for long-running operations
- **FR4.3**: Display formatted addresses and amounts
- **FR4.4**: Provide comprehensive help and error messages

### Non-Functional Requirements

#### NFR1: Performance
- Wallet operations must complete within 30 seconds
- Query operations must complete within 5 seconds
- CLI startup must complete within 1 second
- Memory usage must stay under 50MB

#### NFR2: Security
- Never store private keys persistently
- Validate all inputs before processing
- Use secure communication channels
- Prevent command injection in subprocess calls

#### NFR3: Reliability
- Handle network failures gracefully
- Retry up to 3 times with exponential backoff (configurable via CLI flags, config file, then defaults)
- Maintain transaction atomicity
- Preserve data integrity during operations

#### NFR4: Usability
- Support multiple authentication methods
- Provide consistent command-line interface
- Offer clear error messages with resolution suggestions
- Maintain backward compatibility
- Include optional performance metrics in CLI output (verbosity levels): operation duration, memory usage, gas costs, network latency, retry counts

#### NFR5: Configuration
- Use environment variables for sensitive information (private keys, API keys)
- Support configuration file for non-sensitive settings
- Allow command-line flags to override configuration (precedence: CLI flags → config file → defaults)
- Show configuration source in `config` command output

## System Architecture

### High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Layer     │    │   Library Layer │    │  Blockchain     │
│                 │    │                 │    │  Layer          │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ Commands    │ │◄───┤ │ Deployer    │ │◄───┤ │ TEA Network │ │
│ │ - config    │ │    │ │ - predict   │ │    │ │ - RPC       │ │
│ │ - find      │ │    │ │ - deploy    │ │    │ │ - Contracts │ │
│ │ - deploy    │ │    │ │ - send      │ │    │ │ - Gas       │ │
│ │ - send      │ │    │ │             │ │    │ │             │ │
│ │ - execute   │ │    │ └─────────────┘ │    │ └─────────────┘ │
│ │ - sweep     │ │    │ ┌─────────────┐ │    │                 │
│ │ - status    │ │    │ │ Wallet      │ │    │                 │
│ └─────────────┘ │    │ │ - execute   │ │    │                 │
│ ┌─────────────┐ │    │ │ - sweep     │ │    │                 │
│ │ Auth Layer  │ │    │ │ - sign      │ │    │                 │
│ │ - GPG       │ │◄───┤ │ - query     │ │    │                 │
│ │ - BPB       │ │    │ └─────────────┘ │    │                 │
│ │ - Direct    │ │    │ ┌─────────────┐ │    │                 │
│ └─────────────┘ │    │ │ Utils       │ │    │                 │
└─────────────────┘    │ │ - convert   │ │    │                 │
                       │ │ - validate  │ │    │                 │
                       │ │ - format    │ │    │                 │
                       │ └─────────────┘ │    │                 │
                       └─────────────────┘    └─────────────────┘
```

### Component Responsibilities

#### CLI Layer
- **Command Parsing**: Parse and validate command-line arguments
- **User Interface**: Provide colored output, progress indicators, and error messages
- **Authentication Coordination**: Coordinate between GPG, BPB, and direct key methods
- **Operation Orchestration**: Coordinate complex multi-step operations

#### Library Layer
- **Deployer Module**: Handle wallet contract deployment and prediction
- **Wallet Module**: Manage wallet operations and transaction signing
- **Utils Module**: Provide common utilities for formatting, validation, and conversion
- **Error Handling**: Provide consistent error types and messages

#### Blockchain Layer
- **RPC Communication**: Interface with TEA network RPC endpoints
- **Contract Interaction**: Call smart contract functions and handle responses
- **Transaction Management**: Build, sign, and submit transactions
- **Gas Estimation**: Estimate gas limit, gas price, and total cost in TEA

## Command Specifications

### Core Commands

#### `config`
**Purpose**: Display current configuration and system information

**Usage**: `tea-gpg-wallet config`

**Output**:
```
Default configuration:

RPC URL:
  https://rpc.sepolia.tea.xyz
Deployer address:
  0x1234567890123456789012345678901234567890
```

**Requirements**:
- Display RPC URL from environment/config file/build configuration
- Display deployer contract address
- Show network information if available
- Show configuration source (environment, config file, or build-time)

#### `find`
**Purpose**: Find and display wallet information for a GPG key

**Usage**: 
```bash
tea-gpg-wallet find <key_id>
tea-gpg-wallet find --bpb
tea-gpg-wallet find --gpg <email>
```

**Output**:
```
Predicted address for key ID 95469C7E3DFC90B1:
    0xd7baae85d719c2e8e27a70194471ef4b6b253d33 (deployed)

Balance: 1.5 TEA
```

**Requirements**:
- Support all three key identification methods
- Show deployment status
- Display current balance if deployed
- Handle non-deployed wallets gracefully

#### `deploy`
**Purpose**: Deploy a wallet contract for a GPG key

**Usage**:
```bash
tea-gpg-wallet deploy <key_id>
tea-gpg-wallet deploy --bpb
tea-gpg-wallet deploy --gpg <email>
```

**Requirements**:
- Require PRIVATE_KEY environment variable
- Deploy wallet contract on blockchain
- Confirm deployment success
- Display deployed wallet address

#### `send`
**Purpose**: Send TEA tokens to a GPG wallet (deploy if needed)

**Usage**:
```bash
tea-gpg-wallet send <key_id> <amount>
tea-gpg-wallet send --bpb <amount>
tea-gpg-wallet send --gpg <email> <amount>
```

**Requirements**:
- Require PRIVATE_KEY environment variable
- Deploy wallet if not already deployed
- Send specified amount of TEA
- Show before/after balances
- Support decimal amounts (e.g., "1.5", "0.001")

#### `execute` (NEW)
**Purpose**: Send arbitrary amounts from a GPG wallet using signatures

**Usage**:
```bash
# Simple transfers
tea-gpg-wallet execute <key_id> <to_address> <amount>
tea-gpg-wallet execute --bpb <to_address> <amount>
tea-gpg-wallet execute --gpg <email> <to_address> <amount>

# With optional contract call data
tea-gpg-wallet execute <key_id> <to_address> <amount> --data <hex_string>
```

**Requirements**:
- Use GPG/BPB signatures instead of private key
- Support arbitrary destination addresses
- Support arbitrary amounts (not just sweeping all)
- Support optional contract call data (hex string format, validated hex with 1KB maximum length)
- Require wallet to be already deployed
- Show transaction hash on success

#### `sweep`
**Purpose**: Transfer all funds from a GPG wallet to another address

**Usage**:
```bash
tea-gpg-wallet sweep <key_id> <destination_address>
tea-gpg-wallet sweep --bpb <destination_address>
tea-gpg-wallet sweep --gpg <email> <destination_address>
```

**Requirements**:
- Use GPG/BPB signatures for authentication
- Transfer entire wallet balance
- Support arbitrary destination addresses
- Show transaction hash and new balance
- Handle zero balance gracefully

#### `status` (NEW)
**Purpose**: Show detailed wallet status and information

**Usage**:
```bash
tea-gpg-wallet status <key_id>
tea-gpg-wallet status --bpb
tea-gpg-wallet status --gpg <email>
```

**Output**:
```
Wallet Status for key ID 95469C7E3DFC90B1:

Address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Status: deployed
Balance: 1.5 TEA
Nonce: 3
GPG Verifier: 0xabcdef1234567890abcdef1234567890abcdef12
Transaction Count: 7
```

**Requirements**:
- Display basic wallet configuration information
- Show deployment status, balance, nonce, and transaction count
- Display GPG verifier address (read-only info)
- Handle non-deployed wallets

### Key Identification Methods

#### Direct Key ID
- **Format**: 16-character hexadecimal string
- **Example**: `95469C7E3DFC90B1`
- **Validation**: Must be valid hex, exactly 16 characters

#### BPB Integration
- **Flag**: `--bpb` or `-b`
- **Requirements**: BPB tool must be installed and configured
- **Usage**: Automatically retrieves key ID from secure enclave

#### GPG Email Lookup
- **Flag**: `--gpg <email>` or `-g <email>`
- **Requirements**: GPG must be installed with accessible keyring
- **Usage**: Looks up GPG key ID by email address

## API Design

### Library API

#### Deployer Module
```rust
pub mod deployer {
    // Predict wallet address for a key ID
    pub async fn predict_address(key_id: &str) -> Result<AddressPrediction>;
    
    // Deploy wallet contract for a key ID
    pub async fn ensure_deployed(key_id: &str, private_key: &str) -> Result<AddressPrediction>;
    
    // Send TEA to a GPG wallet (deploy if needed)
    pub async fn send_to_gpg_key(key_id: &str, amount: U256, private_key: &str) -> Result<U256>;
    
    // Get balance for a key ID
    pub async fn get_key_id_balance(key_id: &str) -> Result<U256>;
    
    // Get deployer contract address
    pub fn get_contract_address() -> Result<Address>;
}
```

#### Wallet Module
```rust
pub mod wallet {
    // Get signable hash for withdrawal
    pub async fn get_signable_hash(key_id: &str, to: &str) -> Result<SigningData>;
    
    // Get signable hash for execution
    pub async fn get_execute_signable_hash(
        key_id: &str, 
        to: &str, 
        amount: U256,
        data: Option<Bytes>
    ) -> Result<SigningData>;
    
    // Sweep all funds from wallet
    pub async fn sweep_gpg_key(
        key_id: &str,
        to: &str,
        deadline: U256,
        public_key: &str,
        signature: &str,
        private_key: &str,
    ) -> Result<TxHash>;
    
    // Execute arbitrary transaction
    pub async fn execute_gpg_transaction(
        key_id: &str,
        to: &str,
        amount: U256,
        data: Option<Bytes>,
        deadline: U256,
        public_key: &str,
        signature: &str,
        private_key: &str,
    ) -> Result<TxHash>;
    
    // Get wallet status
    pub async fn get_wallet_status(key_id: &str) -> Result<WalletStatus>;
}
```

#### Utils Module
```rust
pub mod utils {
    // Convert decimal string to wei
    pub fn decimal_to_wei_precise(amount_str: &str) -> Result<U256>;
    
    // Convert wei to formatted ETH string
    pub fn wei_to_eth_auto(wei: U256) -> String;
    
    // Convert key ID to bytes
    pub fn key_id_to_bytes(key_id: &str) -> Result<FixedBytes<8>>;
    
    // Get RPC URL
    pub fn get_rpc_url() -> Result<Url>;
    
    // Validate Ethereum address
    pub fn validate_address(address: &str) -> Result<Address>;
}
```

### Data Models

#### Core Types
```rust
#[derive(Debug, Clone)]
pub struct AddressPrediction {
    pub wallet_address: Address,
    pub is_deployed: bool,
}

#[derive(Debug, Clone)]
pub enum SigningData {
    Withdraw {
        blob: FixedBytes<32>,
        deadline: U256,
    },
    Execute {
        blob: FixedBytes<32>,
        deadline: U256,
        to: Address,
        amount: U256,
        data: Option<Bytes>,
    },
}

#[derive(Debug, Clone)]
pub struct WalletStatus {
    pub address: Address,
    pub is_deployed: bool,
    pub balance: U256,
    pub nonce: U256,
    pub gpg_verifier: Address,
    pub transaction_count: U256,
}

#[derive(Debug, Clone)]
pub struct SigningResult {
    pub signature: String,
    pub public_key: String,
}
```

#### Error Types
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
}
```

## Error Handling

### Error Categories

#### User Errors (Exit Code 2)
- Invalid command arguments
- Missing required parameters
- Invalid key IDs or addresses
- Insufficient permissions

#### System Errors (Exit Code 1)
- Network connectivity issues
- Blockchain transaction failures
- GPG/BPB tool failures
- Configuration problems

#### Security Errors (Exit Code 3)
- Signature verification failures
- Invalid authentication
- Security policy violations

### Error Message Format
```
Error: <brief description>

Details: <detailed explanation>

Suggestions:
  - <suggestion 1>
  - <suggestion 2>

For more help, run: tea-gpg-wallet <command> --help
```

### Error Recovery
- Retry up to 3 times with exponential backoff for network failures
- Configuration precedence: CLI flags → config file → defaults
- Provide retry suggestions for transient failures
- Offer alternative authentication methods
- Suggest configuration fixes
- Include relevant documentation links

## Security Considerations

### Input Validation
- Validate all GPG key IDs (16 hex characters)
- Validate Ethereum addresses (checksum format)
- Validate TEA amounts (positive, within precision limits)
- Validate contract call data (hex format, maximum 1KB length)
- Sanitize all user inputs before processing

### Authentication Security
- Never store private keys or sensitive data
- Use secure communication channels (HTTPS)
- Validate GPG signatures before accepting transactions
- Implement proper key verification

### Subprocess Security
- Prevent command injection in GPG/BPB calls
- Use secure argument passing
- Validate subprocess outputs
- Implement timeout mechanisms

### Transaction Security
- Verify transaction parameters before signing
- Implement deadline validation
- Use proper nonce management
- Validate contract addresses

## Performance Requirements

### Response Time Targets
- **Wallet Operations**: ≤ 30 seconds (deploy, send, execute, sweep)
- **Query Operations**: ≤ 5 seconds (find, status, balance)
- **CLI Startup**: ≤ 1 second
- **Help/Config**: ≤ 100ms

### Resource Usage Limits
- **Memory**: ≤ 50MB for typical operations
- **CPU**: Minimize usage during idle periods
- **Network**: Batch requests, implement connection pooling
- **Disk**: Minimal temporary file usage

### Scalability Considerations
- Support concurrent operations
- Handle large TEA amounts without precision loss
- Efficiently manage multiple GPG keys
- Implement proper connection management

## Testing Strategy

### Unit Testing
- **Coverage Target**: 80% line coverage minimum
- **Test Categories**: 
  - Input validation
  - Amount conversion
  - Address formatting
  - Error handling
- **Mocking**: Mock external dependencies (GPG, BPB, RPC)

### Integration Testing
- **CLI Commands**: Test all commands with real GPG keys (read from environment if present, pass test if absent)
- **End-to-End**: Test complete workflows
- **Error Paths**: Test all error conditions
- **Authentication**: Test all authentication methods

### Security Testing
- **Input Validation**: Test with malicious inputs
- **Authentication**: Test signature verification
- **Subprocess**: Test command injection prevention
- **Error Messages**: Ensure no sensitive data leakage

### Performance Testing
- **Load Testing**: Test with multiple concurrent operations
- **Memory Testing**: Monitor memory usage patterns
- **Network Testing**: Test with various network conditions
- **Timeout Testing**: Test operation timeouts

## Implementation Plan

### Phase 1: Core Infrastructure (Weeks 1-2)
- [ ] Implement basic CLI structure
- [ ] Add configuration management
- [ ] Implement key identification methods
- [ ] Add basic error handling

### Phase 2: Basic Wallet Operations (Weeks 3-4)
- [ ] Implement `find` command
- [ ] Implement `deploy` command
- [ ] Implement `send` command
- [ ] Add progress indicators

### Phase 3: Advanced Operations (Weeks 5-6)
- [ ] Implement `execute` command
- [ ] Implement `sweep` command
- [ ] Implement `status` command
- [ ] Add transaction signing

### Phase 4: Polish and Testing (Weeks 7-8)
- [ ] Comprehensive testing
- [ ] Error handling improvements
- [ ] Documentation completion
- [ ] Performance optimization

### Phase 5: Release Preparation (Weeks 9-10)
- [ ] Security audit
- [ ] Performance testing
- [ ] Documentation review
- [ ] Release preparation

## Success Criteria

### Functional Success
- All specified commands work correctly
- All authentication methods function properly
- Error handling provides clear guidance
- Performance meets specified targets

### Quality Success
- 80%+ test coverage achieved
- No critical security vulnerabilities
- Code passes all linting checks
- Documentation is comprehensive

### User Experience Success
- Commands are intuitive and consistent
- Error messages are helpful and actionable
- Performance is acceptable for typical usage
- Installation and setup are straightforward

## Clarifications

### Session 2024-12-19
- Q: Should the `execute` command support smart contract calls beyond simple transfers? → A: Simple transfers + optional contract call data initially; full ABI support eventually
- Q: What admin information should the `status` command display? → A: Should show basic configuration pieces
- Q: How detailed should gas estimation be? → A: Gas limit + price + total cost in TEA
- Q: How aggressive should error recovery retry behavior be? → A: Retry up to 3 times with exponential backoff by default, configurable
- Q: How should configuration be managed? → A: Environment variables + config file + command line flags; env for sensitive info
- Q: What validation rules should apply to the `--data` parameter in the execute command? → A: Validate hex format + maximum length (e.g., 1KB limit)
- Q: What specific fields should the `status` command display for wallet information? → A: Address, status, balance, nonce, GPG verifier, transaction count
- Q: What environment variable names should control the retry behavior? → A: CLI flags override config file override defaults (precedence order)
- Q: How should the SigningData model handle different signing scenarios (withdraw vs execute)? → A: Create separate SigningData variants for different operations
- Q: What performance monitoring capabilities should be implemented? → A: Add performance metrics to CLI output (operation timing, memory usage) with optional verbosity
- Q: Should the `sweep` command support direct GPG key ID specification? → A: Add direct key ID support with flag for consistency
- Q: What specific performance metrics should be displayed in CLI output? → A: Detailed metrics: operation duration, memory usage, gas costs, network latency, retry counts
- Q: How should retry configuration be managed? → A: CLI flags first, then config file, then defaults
- Q: How should test GPG keys and fixtures be handled? → A: Read from environment if present, pass test if absent
- Q: Should implementation plan phases have specific deliverables defined? → A: Keep current high-level approach, deliverables defined during planning

---

*This specification serves as the authoritative guide for implementing the TEA GPG Wallet CLI tool. All implementation decisions should align with these requirements and principles.*
