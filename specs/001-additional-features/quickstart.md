# Quickstart Guide: TEA GPG Wallet CLI Tool

**Date**: 2024-12-19  
**Feature**: TEA GPG Wallet CLI Tool  
**Purpose**: Complete user workflow validation and testing scenarios

## Prerequisites

### Required Tools
- **Rust 1.89+**: For building the CLI tool
- **GPG**: For GPG key operations (optional, can use BPB instead)
- **BPB**: For secure enclave operations (optional, can use GPG instead)
- **TEA Testnet Access**: For blockchain operations

### Environment Setup
```bash
# Set up environment variables
export PRIVATE_KEY="0x..."  # Your private key for deployment/sending
export RPC_URL="https://rpc.sepolia.tea.xyz"  # TEA testnet RPC
export DEPLOYER_ADDRESS="0x..."  # GPG deployer contract address
```

## Installation

```bash
# Clone and build
git clone <repository>
cd tea-gpg-wallet
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Basic Workflow

### 1. Configuration Check
```bash
# Verify configuration
tea-gpg-wallet config
```

**Expected Output**:
```
Default configuration:

RPC URL:
  https://rpc.sepolia.tea.xyz
Deployer address:
  0x1234567890123456789012345678901234567890
Network: Sepolia Testnet
Configuration source: environment variables
```

### 2. Find Wallet Address
```bash
# Using direct key ID
tea-gpg-wallet find 95469C7E3DFC90B1

# Using BPB
tea-gpg-wallet find --bpb

# Using GPG email
tea-gpg-wallet find --gpg user@example.com
```

**Expected Output**:
```
Predicted address for key ID 95469C7E3DFC90B1:
    0xd7baae85d719c2e8e27a70194471ef4b6b253d33 (not deployed)

Balance: 0 TEA
```

### 3. Deploy Wallet (if needed)
```bash
# Deploy wallet for key
tea-gpg-wallet deploy 95469C7E3DFC90B1
```

**Expected Output**:
```
Deploying wallet for key ID 95469C7E3DFC90B1...
✓ Wallet deployed successfully
Address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
```

### 4. Send TEA to Wallet
```bash
# Send 1.5 TEA to wallet
tea-gpg-wallet send 95469C7E3DFC90B1 1.5
```

**Expected Output**:
```
Sending 1.5 TEA to key ID 95469C7E3DFC90B1...
✓ Wallet deployed (was not deployed)
✓ TEA sent successfully
Recipient address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 1.5 TEA
```

### 5. Check Wallet Status
```bash
# Get detailed wallet information
tea-gpg-wallet status 95469C7E3DFC90B1
```

**Expected Output**:
```
Wallet Status for key ID 95469C7E3DFC90B1:

Address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Status: deployed
Balance: 1.5 TEA
Nonce: 1
GPG Verifier: 0xabcdef1234567890abcdef1234567890abcdef12
Transaction Count: 1
```

### 6. Execute Transaction with GPG Signature
```bash
# Send 0.5 TEA to another address
tea-gpg-wallet execute 95469C7E3DFC90B1 0xabcdef1234567890abcdef1234567890abcdef12 0.5
```

**Expected Output**:
```
Executing transaction for key ID 95469C7E3DFC90B1...
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 0.5 TEA
Data: (none)
✓ Transaction executed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 1.0 TEA
```

### 7. Execute Transaction with Contract Data
```bash
# Send with contract call data
tea-gpg-wallet execute 95469C7E3DFC90B1 0xabcdef1234567890abcdef1234567890abcdef12 0.1 --data 0x1234abcd
```

**Expected Output**:
```
Executing transaction for key ID 95469C7E3DFC90B1...
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 0.1 TEA
Data: 0x1234abcd
✓ Transaction executed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 0.9 TEA
```

### 8. Sweep All Funds
```bash
# Transfer all remaining funds
tea-gpg-wallet sweep 95469C7E3DFC90B1 0xabcdef1234567890abcdef1234567890abcdef12
```

**Expected Output**:
```
Sweeping wallet for key ID 95469C7E3DFC90B1...
From: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 0.9 TEA (all funds)
✓ Sweep completed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 0 TEA
```

## Advanced Workflows

### Multiple Authentication Methods
```bash
# Test all authentication methods
tea-gpg-wallet find 95469C7E3DFC90B1
tea-gpg-wallet find --bpb
tea-gpg-wallet find --gpg user@example.com
```

### Configuration Management
```bash
# Override configuration
tea-gpg-wallet --rpc-url https://custom-rpc.tea.xyz find 95469C7E3DFC90B1
tea-gpg-wallet --retry-count 5 deploy 95469C7E3DFC90B1
```

### Verbose Output
```bash
# Enable performance metrics
tea-gpg-wallet --verbose execute 95469C7E3DFC90B1 0x... 0.1
```

**Expected Output**:
```
Executing transaction for key ID 95469C7E3DFC90B1...
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 0.1 TEA
Data: (none)
[2.3s] Getting signable hash...
[1.8s] GPG signing...
[4.1s] Submitting transaction...
✓ Transaction executed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 0.8 TEA
Total time: 8.2s
```

### JSON Output
```bash
# Machine-readable output
tea-gpg-wallet --json status 95469C7E3DFC90B1
```

**Expected Output**:
```json
{
  "address": "0xd7baae85d719c2e8e27a70194471ef4b6b253d33",
  "is_deployed": true,
  "balance": "800000000000000000",
  "nonce": "3",
  "gpg_verifier": "0xabcdef1234567890abcdef1234567890abcdef12",
  "transaction_count": "3"
}
```

## Error Scenarios

### Invalid Key ID
```bash
tea-gpg-wallet find invalid-key
```

**Expected Output**:
```
Error: Invalid key ID format

Details: Key ID must be exactly 16 hexadecimal characters

Suggestions:
  - Use a valid GPG key ID (e.g., 95469C7E3DFC90B1)
  - Use --gpg <email> to lookup by email
  - Use --bpb for BPB integration

For more help, run: tea-gpg-wallet find --help
```

### Wallet Not Deployed
```bash
tea-gpg-wallet execute 95469C7E3DFC90B1 0x... 0.1
```

**Expected Output**:
```
Error: Wallet not deployed

Details: Wallet contract not deployed for key ID 95469C7E3DFC90B1

Suggestions:
  - Deploy wallet first: tea-gpg-wallet deploy 95469C7E3DFC90B1
  - Send TEA to auto-deploy: tea-gpg-wallet send 95469C7E3DFC90B1 0.1

For more help, run: tea-gpg-wallet execute --help
```

### Insufficient Balance
```bash
tea-gpg-wallet send 95469C7E3DFC90B1 1000
```

**Expected Output**:
```
Error: Insufficient balance

Details: Required 1000 TEA, available 0.9 TEA

Suggestions:
  - Reduce amount to 0.9 TEA or less
  - Add more TEA to your account first
  - Check balance: tea-gpg-wallet status 95469C7E3DFC90B1

For more help, run: tea-gpg-wallet send --help
```

## Testing Scenarios

### Complete Workflow Test
1. **Setup**: Configure environment and install tool
2. **Find**: Predict wallet address for test key
3. **Deploy**: Deploy wallet contract
4. **Send**: Send initial TEA amount
5. **Status**: Verify wallet state
6. **Execute**: Perform GPG-signed transaction
7. **Sweep**: Transfer all funds
8. **Verify**: Confirm final state

### Authentication Method Test
1. **Direct Key**: Test with explicit key ID
2. **BPB**: Test with BPB integration
3. **GPG Email**: Test with email lookup
4. **Error Handling**: Test with invalid inputs

### Error Path Test
1. **Invalid Inputs**: Test validation errors
2. **Network Failures**: Test retry logic
3. **Authentication Failures**: Test signature errors
4. **Balance Issues**: Test insufficient balance

### Performance Test
1. **Startup Time**: Measure CLI startup
2. **Operation Times**: Measure wallet operations
3. **Memory Usage**: Monitor resource consumption
4. **Concurrent Operations**: Test parallel execution

## Success Criteria

### Functional Success
- All commands work correctly with valid inputs
- All authentication methods function properly
- Error handling provides clear guidance
- Performance meets specified targets

### User Experience Success
- Commands are intuitive and consistent
- Error messages are helpful and actionable
- Progress indicators work for long operations
- Installation and setup are straightforward

### Quality Success
- No critical security vulnerabilities
- Code passes all linting checks
- Documentation is comprehensive
- Tests cover all scenarios

---

*Quickstart guide provides complete validation scenarios for the TEA GPG Wallet CLI tool.*