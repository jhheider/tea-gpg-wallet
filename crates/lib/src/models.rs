//! Data models for the TEA GPG Wallet CLI Tool
//!
//! This module defines the core data structures used throughout the application,
//! including AddressPrediction, SigningData, WalletStatus, and SigningResult.

use alloy_primitives::{
    Address,
    Bytes,
    FixedBytes,
    U256,
};
use serde::{
    Deserialize,
    Serialize,
};

/// Represents a predicted or deployed wallet address for a GPG key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressPrediction {
    /// Ethereum address (20 bytes)
    pub wallet_address: Address,
    /// Whether contract is deployed
    pub is_deployed: bool,
}

/// Represents data to be signed for different wallet operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SigningData {
    /// Withdrawal operation data
    Withdraw {
        /// EIP-712 hash blob
        blob: FixedBytes<32>,
        /// Transaction deadline
        deadline: U256,
    },
    /// Execute operation data
    Execute {
        /// EIP-712 hash blob
        blob: FixedBytes<32>,
        /// Transaction deadline
        deadline: U256,
        /// Destination address
        to: Address,
        /// Amount to transfer
        amount: U256,
        /// Optional contract call data (max 1KB)
        data: Option<Bytes>,
    },
}

/// Represents complete wallet state and configuration information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletStatus {
    /// Wallet contract address
    pub address: Address,
    /// Deployment status
    pub is_deployed: bool,
    /// Current TEA balance (wei)
    pub balance: U256,
    /// Next transaction nonce
    pub nonce: U256,
    /// GPG verifier contract address
    pub gpg_verifier: Address,
    /// Total transaction count
    pub transaction_count: U256,
}

/// Represents the result of a GPG/BPB signing operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigningResult {
    /// Hex-encoded signature
    pub signature: String,
    /// Hex-encoded public key
    pub public_key: String,
}
