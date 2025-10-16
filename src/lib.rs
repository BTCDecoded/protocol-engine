//! Bitcoin Protocol Engine
//! 
//! This crate provides a Bitcoin protocol abstraction layer that enables:
//! - Multiple Bitcoin variants (mainnet, testnet, regtest, educational)
//! - Protocol evolution support (Bitcoin V1, V2, etc.)
//! - Economic model abstraction (PoW, future variants)
//! - Educational and research-friendly interfaces
//! 
//! This is Tier 3 of the 5-tier BTCDecoded architecture:
//! 
//! 1. Orange Paper (mathematical foundation)
//! 2. consensus-proof (pure math implementation)
//! 3. protocol-engine (Bitcoin abstraction) ← THIS CRATE
//! 4. reference-node (full Bitcoin node)
//! 5. developer-sdk (ergonomic API)

use consensus_proof::{ConsensusProof, Result, ValidationResult, Block, Transaction};
use serde::{Deserialize, Serialize};

pub mod variants;
pub mod validation;
pub mod network_params;

/// Bitcoin Protocol Engine
/// 
/// Provides protocol abstraction for different Bitcoin variants and evolution.
/// Acts as a bridge between consensus-proof (pure math) and reference-node (implementation).
pub struct BitcoinProtocolEngine {
    consensus: ConsensusProof,
    protocol_version: ProtocolVersion,
    network_params: NetworkParameters,
}

/// Bitcoin protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolVersion {
    /// Current Bitcoin mainnet protocol
    BitcoinV1,
    /// Bitcoin testnet protocol
    Testnet3,
    /// Regression test network protocol
    Regtest,
}

/// Network parameters for different Bitcoin variants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkParameters {
    /// Network magic bytes for P2P protocol
    pub magic_bytes: [u8; 4],
    /// Default P2P port
    pub default_port: u16,
    /// Genesis block for this network
    pub genesis_block: Block,
    /// Maximum proof-of-work target
    pub max_target: u32,
    /// Block subsidy halving interval
    pub halving_interval: u64,
    /// Network name for identification
    pub network_name: String,
    /// Whether this is a test network
    pub is_testnet: bool,
}

impl BitcoinProtocolEngine {
    /// Create a new protocol engine for the specified variant
    pub fn new(version: ProtocolVersion) -> Result<Self> {
        let consensus = ConsensusProof::new();
        let network_params = NetworkParameters::for_version(version)?;
        
        Ok(BitcoinProtocolEngine {
            consensus,
            protocol_version: version,
            network_params,
        })
    }
    
    /// Get the current protocol version
    pub fn get_protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
    
    /// Get network parameters for this protocol
    pub fn get_network_params(&self) -> &NetworkParameters {
        &self.network_params
    }
    
    /// Validate a block using this protocol's rules
    pub fn validate_block(&self, block: &Block, utxos: &std::collections::HashMap<consensus_proof::types::OutPoint, consensus_proof::types::UTXO>, height: u64) -> Result<ValidationResult> {
        let (result, _) = self.consensus.validate_block(block, utxos.clone(), height)?;
        Ok(result)
    }
    
    /// Validate a transaction using this protocol's rules
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<ValidationResult> {
        self.consensus.validate_transaction(tx)
    }
    
    /// Check if this protocol supports a specific feature
    pub fn supports_feature(&self, feature: &str) -> bool {
        match self.protocol_version {
            ProtocolVersion::BitcoinV1 => {
                matches!(feature, "segwit" | "taproot" | "rbf" | "ctv")
            }
            ProtocolVersion::Testnet3 => {
                matches!(feature, "segwit" | "taproot" | "rbf" | "ctv")
            }
            ProtocolVersion::Regtest => {
                matches!(feature, "segwit" | "taproot" | "rbf" | "ctv" | "fast_mining")
            }
        }
    }
}

impl NetworkParameters {
    /// Create network parameters for a specific protocol version
    pub fn for_version(version: ProtocolVersion) -> Result<Self> {
        match version {
            ProtocolVersion::BitcoinV1 => Self::mainnet(),
            ProtocolVersion::Testnet3 => Self::testnet(),
            ProtocolVersion::Regtest => Self::regtest(),
        }
    }
    
    /// Create a placeholder block for testing
    fn create_placeholder_block() -> Block {
        // TODO: Create proper genesis blocks
        // For now, return a minimal block structure
        use consensus_proof::types::*;
        Block {
            header: BlockHeader {
                version: 1,
                prev_block_hash: [0u8; 32],
                merkle_root: [0u8; 32],
                timestamp: 1231006505,
                bits: 0x1d00ffff,
                nonce: 0,
            },
            transactions: vec![],
        }
    }
    
    /// Bitcoin mainnet parameters
    pub fn mainnet() -> Result<Self> {
        // TODO: Implement actual mainnet genesis block and parameters
        // For now, return placeholder values
        Ok(NetworkParameters {
            magic_bytes: [0xf9, 0xbe, 0xb4, 0xd9], // Bitcoin mainnet magic
            default_port: 8333,
            genesis_block: Self::create_placeholder_block(), // TODO: Real genesis block
            max_target: 0x1d00ffff,
            halving_interval: 210000,
            network_name: "mainnet".to_string(),
            is_testnet: false,
        })
    }
    
    /// Bitcoin testnet parameters
    pub fn testnet() -> Result<Self> {
        Ok(NetworkParameters {
            magic_bytes: [0x0b, 0x11, 0x09, 0x07], // Bitcoin testnet magic
            default_port: 18333,
            genesis_block: Self::create_placeholder_block(), // TODO: Real testnet genesis block
            max_target: 0x1d00ffff,
            halving_interval: 210000,
            network_name: "testnet".to_string(),
            is_testnet: true,
        })
    }
    
    /// Bitcoin regtest parameters
    pub fn regtest() -> Result<Self> {
        Ok(NetworkParameters {
            magic_bytes: [0xfa, 0xbf, 0xb5, 0xda], // Bitcoin regtest magic
            default_port: 18444,
            genesis_block: Self::create_placeholder_block(), // TODO: Real regtest genesis block
            max_target: 0x207fffff, // Easier difficulty for testing
            halving_interval: 150, // Faster halving for testing
            network_name: "regtest".to_string(),
            is_testnet: true,
        })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_protocol_engine_creation() {
        let engine = BitcoinProtocolEngine::new(ProtocolVersion::BitcoinV1).unwrap();
        assert_eq!(engine.get_protocol_version(), ProtocolVersion::BitcoinV1);
        assert_eq!(engine.get_network_params().network_name, "mainnet");
    }
    
    #[test]
    fn test_network_parameters() {
        let mainnet = NetworkParameters::mainnet().unwrap();
        assert_eq!(mainnet.magic_bytes, [0xf9, 0xbe, 0xb4, 0xd9]);
        assert_eq!(mainnet.default_port, 8333);
        assert!(!mainnet.is_testnet);
        
        let testnet = NetworkParameters::testnet().unwrap();
        assert_eq!(testnet.magic_bytes, [0x0b, 0x11, 0x09, 0x07]);
        assert_eq!(testnet.default_port, 18333);
        assert!(testnet.is_testnet);
    }
    
    #[test]
    fn test_feature_support() {
        let mainnet = BitcoinProtocolEngine::new(ProtocolVersion::BitcoinV1).unwrap();
        assert!(mainnet.supports_feature("segwit"));
        assert!(mainnet.supports_feature("taproot"));
        assert!(!mainnet.supports_feature("nonexistent"));
        
        let regtest = BitcoinProtocolEngine::new(ProtocolVersion::Regtest).unwrap();
        assert!(regtest.supports_feature("fast_mining"));
        assert!(regtest.supports_feature("segwit"));
    }
}
