//! Network parameters for different Bitcoin variants
//! 
//! This module defines the network-specific parameters for different
//! Bitcoin protocol variants, including magic bytes, ports, genesis blocks,
//! and other network-specific constants.

use crate::{ProtocolVersion, NetworkParameters, Result};
use serde::{Deserialize, Serialize};

/// Network-specific constants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkConstants {
    /// Network magic bytes for P2P protocol identification
    pub magic_bytes: [u8; 4],
    /// Default P2P port for this network
    pub default_port: u16,
    /// Genesis block hash for this network
    pub genesis_hash: [u8; 32],
    /// Maximum proof-of-work target
    pub max_target: u32,
    /// Block subsidy halving interval
    pub halving_interval: u64,
    /// Network name for identification
    pub network_name: String,
    /// Whether this is a test network
    pub is_testnet: bool,
    /// DNS seeds for peer discovery
    pub dns_seeds: Vec<String>,
    /// Checkpoint blocks for fast sync
    pub checkpoints: Vec<Checkpoint>,
}

/// Checkpoint block for fast synchronization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Block height
    pub height: u64,
    /// Block hash
    pub hash: [u8; 32],
    /// Block timestamp
    pub timestamp: u64,
}

impl NetworkConstants {
    /// Get constants for a specific protocol version
    pub fn for_version(version: ProtocolVersion) -> Result<Self> {
        match version {
            ProtocolVersion::BitcoinV1 => Self::mainnet(),
            ProtocolVersion::Testnet3 => Self::testnet(),
            ProtocolVersion::Regtest => Self::regtest(),
        }
    }
    
    /// Bitcoin mainnet constants
    pub fn mainnet() -> Result<Self> {
        Ok(Self {
            magic_bytes: [0xf9, 0xbe, 0xb4, 0xd9], // Bitcoin mainnet magic
            default_port: 8333,
            genesis_hash: [
                0x6f, 0xe2, 0x8c, 0x0a, 0xb6, 0xf1, 0xb3, 0x72,
                0xc1, 0xa6, 0xa2, 0x46, 0xae, 0x63, 0xf7, 0x4f,
                0x93, 0x1e, 0x83, 0x65, 0xe1, 0x5a, 0x08, 0x9c,
                0x68, 0xd6, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00
            ],
            max_target: 0x1d00ffff,
            halving_interval: 210000,
            network_name: "mainnet".to_string(),
            is_testnet: false,
            dns_seeds: vec![
                "seed.bitcoin.sipa.be".to_string(),
                "dnsseed.bluematt.me".to_string(),
                "dnsseed.bitcoin.dashjr.org".to_string(),
                "seed.bitcoinstats.com".to_string(),
                "seed.bitcoin.jonasschnelli.ch".to_string(),
                "seed.btc.petertodd.org".to_string(),
            ],
            checkpoints: Self::mainnet_checkpoints(),
        })
    }
    
    /// Bitcoin testnet constants
    pub fn testnet() -> Result<Self> {
        Ok(Self {
            magic_bytes: [0x0b, 0x11, 0x09, 0x07], // Bitcoin testnet magic
            default_port: 18333,
            genesis_hash: [
                0x43, 0x49, 0x7f, 0xd7, 0xf8, 0x26, 0x95, 0x71,
                0x08, 0xf4, 0xa3, 0x0f, 0xd9, 0xce, 0xc3, 0xae,
                0xba, 0x79, 0x97, 0x20, 0x84, 0xe9, 0x0e, 0xad,
                0x01, 0xea, 0x33, 0x09, 0x00, 0x00, 0x00, 0x00
            ],
            max_target: 0x1d00ffff,
            halving_interval: 210000,
            network_name: "testnet".to_string(),
            is_testnet: true,
            dns_seeds: vec![
                "testnet-seed.bitcoin.jonasschnelli.ch".to_string(),
                "seed.tbtc.petertodd.org".to_string(),
                "seed.testnet.bitcoin.sprovoost.nl".to_string(),
                "testnet-seed.bluematt.me".to_string(),
            ],
            checkpoints: Self::testnet_checkpoints(),
        })
    }
    
    /// Bitcoin regtest constants
    pub fn regtest() -> Result<Self> {
        Ok(Self {
            magic_bytes: [0xfa, 0xbf, 0xb5, 0xda], // Bitcoin regtest magic
            default_port: 18444,
            genesis_hash: [
                0x06, 0x22, 0x6e, 0x46, 0x11, 0x1a, 0x0b, 0x59,
                0xca, 0xaf, 0x12, 0x60, 0x43, 0xeb, 0x5b, 0xbf,
                0x28, 0xc3, 0x4f, 0x3a, 0x5e, 0x33, 0x2a, 0x1f,
                0xc7, 0xb2, 0xb7, 0x3c, 0xf1, 0x88, 0x91, 0x0f
            ],
            max_target: 0x207fffff, // Easier difficulty for testing
            halving_interval: 150,   // Faster halving for testing
            network_name: "regtest".to_string(),
            is_testnet: true,
            dns_seeds: vec![], // No DNS seeds for regtest
            checkpoints: vec![], // No checkpoints for regtest
        })
    }
    
    /// Mainnet checkpoints for fast sync
    fn mainnet_checkpoints() -> Vec<Checkpoint> {
        vec![
            Checkpoint {
                height: 11111,
                hash: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ],
                timestamp: 1231006505,
            },
            // Add more checkpoints as needed
        ]
    }
    
    /// Testnet checkpoints for fast sync
    fn testnet_checkpoints() -> Vec<Checkpoint> {
        vec![
            Checkpoint {
                height: 11111,
                hash: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ],
                timestamp: 1296688602,
            },
            // Add more checkpoints as needed
        ]
    }
}

impl NetworkParameters {
    /// Create network parameters from constants
    pub fn from_constants(constants: &NetworkConstants) -> Result<Self> {
        Ok(NetworkParameters {
            magic_bytes: constants.magic_bytes,
            default_port: constants.default_port,
            genesis_block: NetworkParameters::create_placeholder_block(), // TODO: Create actual genesis block
            max_target: constants.max_target,
            halving_interval: constants.halving_interval,
            network_name: constants.network_name.clone(),
            is_testnet: constants.is_testnet,
        })
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_constants() {
        let mainnet = NetworkConstants::mainnet().unwrap();
        assert_eq!(mainnet.magic_bytes, [0xf9, 0xbe, 0xb4, 0xd9]);
        assert_eq!(mainnet.default_port, 8333);
        assert!(!mainnet.is_testnet);
        assert!(!mainnet.dns_seeds.is_empty());
        
        let testnet = NetworkConstants::testnet().unwrap();
        assert_eq!(testnet.magic_bytes, [0x0b, 0x11, 0x09, 0x07]);
        assert_eq!(testnet.default_port, 18333);
        assert!(testnet.is_testnet);
        
        let regtest = NetworkConstants::regtest().unwrap();
        assert_eq!(regtest.magic_bytes, [0xfa, 0xbf, 0xb5, 0xda]);
        assert_eq!(regtest.default_port, 18444);
        assert!(regtest.is_testnet);
        assert!(regtest.dns_seeds.is_empty());
    }
    
    #[test]
    fn test_network_parameters_from_constants() {
        let constants = NetworkConstants::mainnet().unwrap();
        let params = NetworkParameters::from_constants(&constants).unwrap();
        
        assert_eq!(params.magic_bytes, constants.magic_bytes);
        assert_eq!(params.default_port, constants.default_port);
        assert_eq!(params.network_name, constants.network_name);
        assert_eq!(params.is_testnet, constants.is_testnet);
    }
}
