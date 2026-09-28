# CLI Interface Contract: TEA GPG Wallet CLI Tool

**Date**: 2024-12-19  
**Feature**: TEA GPG Wallet CLI Tool  
**Purpose**: Define CLI command specifications and expected behavior

## Command Overview

All commands follow the pattern: `tea-gpg-wallet <command> [options] [arguments]`

## Authentication Methods

All commands support three authentication methods:
- **Direct Key ID**: `<key_id>` (16-character hex string)
- **BPB Integration**: `--bpb` or `-b`
- **GPG Email Lookup**: `--gpg <email>` or `-g <email>`

## Command Specifications

### config
**Purpose**: Display current configuration and system information

**Usage**: `tea-gpg-wallet config`

**Output Format**:
```
Default configuration:

RPC URL:
  https://rpc.sepolia.tea.xyz
Deployer address:
  0x1234567890123456789012345678901234567890
Network: Sepolia Testnet
Configuration source: environment variables
```

**Requirements**:
- Display RPC URL from configuration
- Display deployer contract address
- Show network information
- Show configuration source (environment, config file, build-time)
- Exit code: 0 on success

**Error Cases**:
- Missing configuration: Exit code 1, error message with setup instructions
- Invalid configuration: Exit code 1, error message with validation details

### find
**Purpose**: Find and display wallet information for a GPG key

**Usage**: 
```bash
tea-gpg-wallet find <key_id>
tea-gpg-wallet find --bpb
tea-gpg-wallet find --gpg <email>
```

**Output Format**:
```
Predicted address for key ID 95469C7E3DFC90B1:
    0xd7baae85d719c2e8e27a70194471ef4b6b253d33 (deployed)

Balance: 1.5 TEA
```

**Requirements**:
- Support all three authentication methods
- Show deployment status (deployed/not deployed)
- Display current balance if deployed
- Handle non-deployed wallets gracefully
- Exit code: 0 on success

**Error Cases**:
- Invalid key ID: Exit code 2, validation error message
- GPG/BPB tool not found: Exit code 1, installation instructions
- Network error: Exit code 1, retry suggestion

### deploy
**Purpose**: Deploy a wallet contract for a GPG key

**Usage**:
```bash
tea-gpg-wallet deploy <key_id>
tea-gpg-wallet deploy --bpb
tea-gpg-wallet deploy --gpg <email>
```

**Output Format**:
```
Deploying wallet for key ID 95469C7E3DFC90B1...
✓ Wallet deployed successfully
Address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
```

**Requirements**:
- Require PRIVATE_KEY environment variable
- Deploy wallet contract on blockchain
- Confirm deployment success
- Display deployed wallet address
- Show transaction hash
- Exit code: 0 on success

**Error Cases**:
- Missing PRIVATE_KEY: Exit code 2, environment variable setup instructions
- Deployment failure: Exit code 1, error details and retry suggestion
- Already deployed: Exit code 0, show existing address

### send
**Purpose**: Send TEA tokens to a GPG wallet (deploy if needed)

**Usage**:
```bash
tea-gpg-wallet send <key_id> <amount>
tea-gpg-wallet send --bpb <amount>
tea-gpg-wallet send --gpg <email> <amount>
```

**Output Format**:
```
Sending 1.5 TEA to key ID 95469C7E3DFC90B1...
✓ Wallet deployed (was not deployed)
✓ TEA sent successfully
Recipient address: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 1.5 TEA
```

**Requirements**:
- Require PRIVATE_KEY environment variable
- Deploy wallet if not already deployed
- Send specified amount of TEA
- Show before/after balances
- Support decimal amounts (e.g., "1.5", "0.001")
- Exit code: 0 on success

**Error Cases**:
- Invalid amount: Exit code 2, validation error message
- Insufficient balance: Exit code 1, balance information
- Network error: Exit code 1, retry suggestion

### execute
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

**Output Format**:
```
Executing transaction for key ID 95469C7E3DFC90B1...
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 0.5 TEA
Data: 0x1234abcd (optional)
✓ Transaction executed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 1.0 TEA
```

**Requirements**:
- Use GPG/BPB signatures instead of private key
- Support arbitrary destination addresses
- Support arbitrary amounts (not just sweeping all)
- Support optional contract call data (hex string format, max 1KB)
- Require wallet to be already deployed
- Show transaction hash on success
- Exit code: 0 on success

**Error Cases**:
- Wallet not deployed: Exit code 1, deployment suggestion
- Invalid destination address: Exit code 2, validation error
- Invalid contract data: Exit code 2, format requirements
- Signature failure: Exit code 3, authentication error

### sweep
**Purpose**: Transfer all funds from a GPG wallet to another address

**Usage**:
```bash
tea-gpg-wallet sweep <key_id> <destination_address>
tea-gpg-wallet sweep --bpb <destination_address>
tea-gpg-wallet sweep --gpg <email> <destination_address>
```

**Output Format**:
```
Sweeping wallet for key ID 95469C7E3DFC90B1...
From: 0xd7baae85d719c2e8e27a70194471ef4b6b253d33
To: 0xabcdef1234567890abcdef1234567890abcdef12
Amount: 1.5 TEA (all funds)
✓ Sweep completed successfully
Transaction: 0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
New balance: 0 TEA
```

**Requirements**:
- Use GPG/BPB signatures for authentication
- Transfer entire wallet balance
- Support arbitrary destination addresses
- Show transaction hash and new balance
- Handle zero balance gracefully
- Exit code: 0 on success

**Error Cases**:
- Zero balance: Exit code 0, informational message
- Wallet not deployed: Exit code 1, deployment suggestion
- Invalid destination: Exit code 2, validation error

### status
**Purpose**: Show detailed wallet status and information

**Usage**:
```bash
tea-gpg-wallet status <key_id>
tea-gpg-wallet status --bpb
tea-gpg-wallet status --gpg <email>
```

**Output Format**:
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
- Exit code: 0 on success

**Error Cases**:
- Invalid key ID: Exit code 2, validation error
- Network error: Exit code 1, retry suggestion

## Global Options

### --help, -h
Display help information for the command or subcommand.

### --version, -v
Display version information.

### --verbose, -V
Enable verbose output with performance metrics and detailed logging.

### --json
Output results in JSON format for machine parsing.

### --config <path>
Specify custom configuration file path.

### --rpc-url <url>
Override RPC URL for this command.

### --retry-count <number>
Override retry count for network operations (default: 3).

### --retry-backoff-ms <number>
Override retry backoff delay in milliseconds (default: 1000).

## Exit Codes

- **0**: Success
- **1**: System error (network, blockchain, tool failures)
- **2**: User error (invalid arguments, missing parameters)
- **3**: Security error (authentication, signature verification)

## Error Message Format

```
Error: <brief description>

Details: <detailed explanation>

Suggestions:
  - <suggestion 1>
  - <suggestion 2>

For more help, run: tea-gpg-wallet <command> --help
```

## Performance Requirements

- **Wallet Operations**: ≤ 30 seconds (deploy, send, execute, sweep)
- **Query Operations**: ≤ 5 seconds (find, status, balance)
- **CLI Startup**: ≤ 1 second
- **Help/Config**: ≤ 100ms

## Security Requirements

- Never store private keys persistently
- Validate all inputs before processing
- Use secure communication channels (HTTPS)
- Prevent command injection in subprocess calls
- Implement proper key verification

---

*CLI interface contract defines all user-facing commands and their expected behavior.*