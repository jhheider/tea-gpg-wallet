# Library API Contract: TEA GPG Wallet CLI Tool

**Date**: 2024-12-19  
**Feature**: TEA GPG Wallet CLI Tool  
**Purpose**: Define internal library API specifications

## Module Structure

The library is organized into three main modules:
- `deployer`: Wallet deployment and prediction operations
- `wallet`: Wallet operations and transaction signing
- `utils`: Common utilities for formatting, validation, and conversion

## Deployer Module

### predict_address
**Purpose**: Predict wallet address for a GPG key ID

```rust
pub async fn predict_address(key_id: &str) -> Result<AddressPrediction, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)

**Returns**: `AddressPrediction` with predicted address and deployment status

**Error Cases**:
- Invalid key ID format
- Network connectivity issues
- RPC endpoint errors

### ensure_deployed
**Purpose**: Deploy wallet contract for a GPG key ID if not already deployed

```rust
pub async fn ensure_deployed(
    key_id: &str, 
    private_key: &str
) -> Result<AddressPrediction, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `private_key`: Private key for deployment transaction

**Returns**: `AddressPrediction` with deployed address and status

**Error Cases**:
- Invalid key ID format
- Invalid private key
- Deployment transaction failure
- Network connectivity issues

### send_to_gpg_key
**Purpose**: Send TEA tokens to a GPG wallet (deploy if needed)

```rust
pub async fn send_to_gpg_key(
    key_id: &str, 
    amount: U256, 
    private_key: &str
) -> Result<U256, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `amount`: Amount to send in wei
- `private_key`: Private key for sending transaction

**Returns**: Transaction hash as `U256`

**Error Cases**:
- Invalid key ID format
- Invalid amount (zero or negative)
- Insufficient balance
- Deployment failure
- Transaction failure

### get_key_id_balance
**Purpose**: Get balance for a GPG key ID

```rust
pub async fn get_key_id_balance(key_id: &str) -> Result<U256, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)

**Returns**: Balance in wei as `U256`

**Error Cases**:
- Invalid key ID format
- Wallet not deployed
- Network connectivity issues

### get_contract_address
**Purpose**: Get deployer contract address

```rust
pub fn get_contract_address() -> Result<Address, WalletError>
```

**Returns**: Deployer contract address

**Error Cases**:
- Configuration error
- Invalid contract address

## Wallet Module

### get_signable_hash
**Purpose**: Get signable hash for withdrawal operation

```rust
pub async fn get_signable_hash(
    key_id: &str, 
    to: &str
) -> Result<SigningData, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `to`: Destination address

**Returns**: `SigningData::Withdraw` with hash blob and deadline

**Error Cases**:
- Invalid key ID format
- Invalid destination address
- Wallet not deployed
- Network connectivity issues

### get_execute_signable_hash
**Purpose**: Get signable hash for execute operation

```rust
pub async fn get_execute_signable_hash(
    key_id: &str, 
    to: &str, 
    amount: U256,
    data: Option<Bytes>
) -> Result<SigningData, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `to`: Destination address
- `amount`: Amount to transfer
- `data`: Optional contract call data (max 1KB)

**Returns**: `SigningData::Execute` with hash blob, deadline, and operation parameters

**Error Cases**:
- Invalid key ID format
- Invalid destination address
- Invalid amount (zero or negative)
- Invalid contract data (too large or invalid hex)
- Wallet not deployed

### sweep_gpg_key
**Purpose**: Sweep all funds from a GPG wallet

```rust
pub async fn sweep_gpg_key(
    key_id: &str,
    to: &str,
    deadline: U256,
    public_key: &str,
    signature: &str,
    private_key: &str,
) -> Result<TxHash, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `to`: Destination address
- `deadline`: Transaction deadline (Unix timestamp)
- `public_key`: GPG public key (hex string)
- `signature`: GPG signature (hex string)
- `private_key`: Private key for transaction submission

**Returns**: Transaction hash

**Error Cases**:
- Invalid key ID format
- Invalid destination address
- Expired deadline
- Invalid signature
- Wallet not deployed
- Zero balance
- Transaction failure

### execute_gpg_transaction
**Purpose**: Execute arbitrary transaction from a GPG wallet

```rust
pub async fn execute_gpg_transaction(
    key_id: &str,
    to: &str,
    amount: U256,
    data: Option<Bytes>,
    deadline: U256,
    public_key: &str,
    signature: &str,
    private_key: &str,
) -> Result<TxHash, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)
- `to`: Destination address
- `amount`: Amount to transfer
- `data`: Optional contract call data (max 1KB)
- `deadline`: Transaction deadline (Unix timestamp)
- `public_key`: GPG public key (hex string)
- `signature`: GPG signature (hex string)
- `private_key`: Private key for transaction submission

**Returns**: Transaction hash

**Error Cases**:
- Invalid key ID format
- Invalid destination address
- Invalid amount (zero or negative)
- Invalid contract data
- Expired deadline
- Invalid signature
- Wallet not deployed
- Insufficient balance
- Transaction failure

### get_wallet_status
**Purpose**: Get comprehensive wallet status

```rust
pub async fn get_wallet_status(key_id: &str) -> Result<WalletStatus, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID (16-character hex string)

**Returns**: `WalletStatus` with complete wallet information

**Error Cases**:
- Invalid key ID format
- Network connectivity issues
- RPC endpoint errors

## Utils Module

### decimal_to_wei_precise
**Purpose**: Convert decimal string to wei with precise arithmetic

```rust
pub fn decimal_to_wei_precise(amount_str: &str) -> Result<U256, WalletError>
```

**Parameters**:
- `amount_str`: Decimal string (e.g., "1.5", "0.001")

**Returns**: Amount in wei as `U256`

**Error Cases**:
- Invalid decimal format
- Precision overflow
- Negative amounts

### wei_to_eth_auto
**Purpose**: Convert wei to formatted ETH string

```rust
pub fn wei_to_eth_auto(wei: U256) -> String
```

**Parameters**:
- `wei`: Amount in wei

**Returns**: Formatted string (e.g., "1.5 TEA", "0.001 TEA")

### key_id_to_bytes
**Purpose**: Convert key ID string to bytes

```rust
pub fn key_id_to_bytes(key_id: &str) -> Result<FixedBytes<8>, WalletError>
```

**Parameters**:
- `key_id`: GPG key ID string

**Returns**: 8-byte array representation

**Error Cases**:
- Invalid hex format
- Wrong length (not 16 characters)

### get_rpc_url
**Purpose**: Get RPC URL from configuration

```rust
pub fn get_rpc_url() -> Result<Url, WalletError>
```

**Returns**: RPC URL

**Error Cases**:
- Configuration error
- Invalid URL format

### validate_address
**Purpose**: Validate Ethereum address format

```rust
pub fn validate_address(address: &str) -> Result<Address, WalletError>
```

**Parameters**:
- `address`: Address string

**Returns**: Validated address

**Error Cases**:
- Invalid address format
- Invalid checksum

## Error Handling

All functions return `Result<T, WalletError>` where `WalletError` is a comprehensive error type that includes:

- Context information
- User-friendly error messages
- Actionable suggestions
- No sensitive data leakage

## Performance Requirements

- **Network Operations**: Implement retry logic with exponential backoff
- **Input Validation**: Fast local validation before network calls
- **Error Handling**: Quick error detection and reporting
- **Memory Usage**: Efficient data structures and minimal allocations

## Security Requirements

- **Input Validation**: All inputs validated before processing
- **Error Messages**: No sensitive information in error messages
- **Signature Verification**: All signatures verified before acceptance
- **Deadline Validation**: All deadlines checked for expiration

## Testing Requirements

- **Unit Tests**: All functions must have unit tests
- **Integration Tests**: Network operations must have integration tests
- **Error Path Tests**: All error conditions must be tested
- **Mocking**: External dependencies must be mockable

---

*Library API contract defines all internal interfaces with clear specifications for implementation.*