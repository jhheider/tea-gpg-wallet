<!-- markdownlint-disable MD013 -->
# tea-gpg-wallet

A Rust CLI tool for interacting with GPG-based TEA wallets. This tool allows
you to manage TEA tokens using your GPG key identity, providing a secure way
to handle cryptocurrency transactions without traditional private key
management.

## Features

- 🔐 **GPG-based authentication** - Use your existing GPG keys for wallet operations
- 🚀 **Automatic wallet deployment** - Wallets are deployed on-demand when needed
- 💰 **TEA token support** - Send and receive TEA tokens on the Sepolia testnet
- 🔄 **Sweep functionality** - Transfer all funds from a wallet to another address
- 🎨 **Beautiful CLI interface** - Colored output and progress indicators
- 🔧 **Multiple key sources** - Support for direct key IDs, BPB integration, and GPG email lookup

## Installation

### Prerequisites

- **Rust** (1.89 or later)
- **BPB** (from teaBASE for secure enclave operations, or for simpler gpg operations), or
- **GPG** (for GPG-based operations)

### From Source

```bash
git clone https://github.com/jhheider/tea-gpg-wallet
cd tea-gpg-wallet
cargo install --path cli
```

### Using pkgx (**soon**)

```bash
pkgx install tea-gpg-wallet
```

## Quick Start

1. **Set your private key** (for sending transactions):

```bash
export PRIVATE_KEY="your_ethereum_private_key_here"
```

> [!WARNING]
> You should use a wallet with limited funds in this way. Wallets with substantial value should not have their keys exported.

1. **Check configuration**:

```bash
tea-gpg-wallet config
```

1. **Find your wallet address**:

```bash
# Using a GPG key ID directly
tea-gpg-wallet find --key-id 95469C7E3DFC90B1

# Using BPB (if available)
tea-gpg-wallet find --bpb

# Using GPG email lookup
tea-gpg-wallet find --gpg user@example.com
```

## Commands

### `config`

Display the current configuration including RPC URL and deployer contract address.

```bash
tea-gpg-wallet config
```

### `find`

Find the predicted wallet address for a GPG key ID and check if it's deployed.

```bash
# Using direct key ID
tea-gpg-wallet find --key-id 95469C7E3DFC90B1

# Using BPB
tea-gpg-wallet find --bpb

# Using GPG email
tea-gpg-wallet find --gpg user@example.com
```

### `deploy`

Deploy a wallet contract for a GPG key ID (requires private key).

```bash
tea-gpg-wallet deploy --key-id 95469C7E3DFC90B1
```

### `send`

Send TEA tokens to a GPG wallet (deploys wallet if needed).

```bash
# Send 1.5 TEA
tea-gpg-wallet send --key-id 95469C7E3DFC90B1 1.5

# Send 0.001 TEA using BPB
tea-gpg-wallet send --bpb 0.001
```

### `sweep`

Transfer all funds from a GPG wallet to another address.

```bash
# Sweep to address (requires BPB or GPG for signing)
tea-gpg-wallet sweep --bpb 0x1234567890123456789012345678901234567890
```

## Key ID Sources

The tool supports three ways to specify GPG key IDs:

1. **Direct key ID**: `--key-id 95469C7E3DFC90B1`
2. **BPB integration**: `--bpb` (uses secure enclave with teaBASE)
3. **GPG email lookup**: `--gpg user@example.com`

## Environment Variables

- `PRIVATE_KEY`: Your TEA private key (required for send/deploy operations)

## Gotchas & Important Notes

### ⚠️ Security Considerations

- **Private Key Management**: Your `PRIVATE_KEY` environment variable contains sensitive data. Never commit it to version control or share it.
- **GPG Key Security**: Ensure your GPG keys are properly secured, especially when using them for cryptocurrency operations.
- **Testnet Only**: This tool currently operates on the Sepolia testnet. Real funds are not at risk. Mainnet version will launch with TEA's mainnet.

### 🔧 Technical Limitations

- **Sweep Operations**: Sweeping requires either BPB or GPG for signing. Direct key ID mode cannot be used for sweeps.
- **GPG Integration**: GPG operations require the `gpg` command to be available and properly configured.
- **BPB Requirements**: BPB integration requires the `bpb` tool to be installed and configured. [pkgxdev/teaBASE] is recommended.

### 💡 Usage Tips

- **Amount Formatting**: Amounts are specified in TEA (not wei). Use decimal notation: `1.5`, `0.001`, etc.
- **Address Formatting**: Ethereum addresses can be specified with or without the `0x` prefix.
- **Key ID Format**: GPG key IDs should be 16-character hex strings (e.g., `95469C7E3DFC90B1`).

### 🐛 Common Issues

1. **"No GPG key found for email"**: Ensure the email address is associated with a GPG key in your keyring.
2. **"PRIVATE_KEY environment variable not set"**: Set the environment variable before running commands that require it.
3. **"GPG command failed"**: Verify that GPG is installed and your keyring is accessible.

## Examples

### Complete Workflow

```bash
# 1. Set your private key
export PRIVATE_KEY="0x1234567890abcdef..."

# 2. Check configuration
tea-gpg-wallet config

# 3. Find your wallet address
tea-gpg-wallet find --gpg alice@example.com

# 4. Send some TEA
tea-gpg-wallet send --gpg alice@example.com 2.5

# 5. Check balance (via find command)
tea-gpg-wallet find --gpg alice@example.com

# 6. Sweep funds to another address
tea-gpg-wallet sweep --gpg alice@example.com 0x9876543210fedcba...
```

### Using BPB for Enhanced Security

```bash
# Find wallet using BPB
tea-gpg-wallet find --bpb

# Send funds using BPB
tea-gpg-wallet send --bpb 1.0

# Sweep using BPB (most secure)
tea-gpg-wallet sweep --bpb 0x1234567890123456789012345678901234567890
```

## Development

### Building from Source

```bash
git clone https://github.com/jhheider/tea-gpg-wallet
cd tea-gpg-wallet
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Project Structure

```text
tea-gpg-wallet/
├── cli/          # Command-line interface
├── lib/          # Core library functionality
├── abi/          # Ethereum contract ABIs
└── README.md     # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Alloy](https://github.com/alloy-rs/core) for Ethereum interactions
- Inspired by the [teaxyz/gpg-wallet](https://github.com/teaxyz/gpg-wallet) project
- Uses [BPB](https://github.com/pkgxdev/bpb) for secure enclave operations
