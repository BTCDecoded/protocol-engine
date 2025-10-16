//! Protocol variants implementation
//! 
//! This module implements different Bitcoin protocol variants:
//! - Mainnet: Production Bitcoin network
//! - Testnet: Bitcoin test network
//! - Regtest: Regression testing network

use crate::ProtocolVersion;

/// Protocol variant configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolVariant {
    pub version: ProtocolVersion,
    pub name: String,
    pub description: String,
    pub is_production: bool,
    pub supports_mining: bool,
    pub supports_wallet: bool,
}

impl ProtocolVariant {
    /// Get all available protocol variants
    pub fn all_variants() -> Vec<Self> {
        vec![
            ProtocolVariant {
                version: ProtocolVersion::BitcoinV1,
                name: "Bitcoin Mainnet".to_string(),
                description: "Production Bitcoin network with full consensus rules".to_string(),
                is_production: true,
                supports_mining: true,
                supports_wallet: true,
            },
            ProtocolVariant {
                version: ProtocolVersion::Testnet3,
                name: "Bitcoin Testnet".to_string(),
                description: "Bitcoin test network for development and testing".to_string(),
                is_production: false,
                supports_mining: true,
                supports_wallet: true,
            },
            ProtocolVariant {
                version: ProtocolVersion::Regtest,
                name: "Bitcoin Regtest".to_string(),
                description: "Regression testing network with configurable difficulty".to_string(),
                is_production: false,
                supports_mining: true,
                supports_wallet: true,
            },
        ]
    }
    
    /// Get variant by protocol version
    pub fn for_version(version: ProtocolVersion) -> Option<Self> {
        Self::all_variants().into_iter()
            .find(|v| v.version == version)
    }
    
    /// Check if this variant is suitable for production use
    pub fn is_production_ready(&self) -> bool {
        self.is_production
    }
    
    /// Check if this variant supports mining operations
    pub fn supports_mining_operations(&self) -> bool {
        self.supports_mining
    }
    
    /// Check if this variant supports wallet operations
    pub fn supports_wallet_operations(&self) -> bool {
        self.supports_wallet
    }
}

/// Protocol evolution support
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolEvolution {
    /// Protocol version number
    pub version: u32,
    /// Features enabled in this version
    pub enabled_features: Vec<String>,
    /// Features deprecated in this version
    pub deprecated_features: Vec<String>,
    /// Breaking changes from previous version
    pub breaking_changes: Vec<String>,
}

impl ProtocolEvolution {
    /// Bitcoin V1 (current)
    pub fn bitcoin_v1() -> Self {
        Self {
            version: 1,
            enabled_features: vec![
                "basic_transactions".to_string(),
                "proof_of_work".to_string(),
                "economic_model".to_string(),
                "p2p_networking".to_string(),
            ],
            deprecated_features: vec![],
            breaking_changes: vec![],
        }
    }
    
    /// Bitcoin V2 (future hypothetical)
    pub fn bitcoin_v2() -> Self {
        Self {
            version: 2,
            enabled_features: vec![
                "basic_transactions".to_string(),
                "proof_of_work".to_string(),
                "economic_model".to_string(),
                "p2p_networking".to_string(),
                "advanced_scripting".to_string(),
                "privacy_features".to_string(),
            ],
            deprecated_features: vec![
                "legacy_addresses".to_string(),
            ],
            breaking_changes: vec![
                "new_address_format".to_string(),
                "enhanced_script_engine".to_string(),
            ],
        }
    }
    
    /// Check if a feature is enabled in this protocol version
    pub fn has_feature(&self, feature: &str) -> bool {
        self.enabled_features.contains(&feature.to_string())
    }
    
    /// Check if a feature is deprecated in this protocol version
    pub fn is_deprecated(&self, feature: &str) -> bool {
        self.deprecated_features.contains(&feature.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_protocol_variants() {
        let variants = ProtocolVariant::all_variants();
        assert_eq!(variants.len(), 3);
        
        let mainnet = ProtocolVariant::for_version(ProtocolVersion::BitcoinV1).unwrap();
        assert_eq!(mainnet.name, "Bitcoin Mainnet");
        assert!(mainnet.is_production_ready());
        assert!(mainnet.supports_mining_operations());
        
        let regtest = ProtocolVariant::for_version(ProtocolVersion::Regtest).unwrap();
        assert_eq!(regtest.name, "Bitcoin Regtest");
        assert!(!regtest.is_production_ready());
        assert!(regtest.supports_mining_operations());
    }
    
    #[test]
    fn test_protocol_evolution() {
        let v1 = ProtocolEvolution::bitcoin_v1();
        assert_eq!(v1.version, 1);
        assert!(v1.has_feature("basic_transactions"));
        assert!(!v1.is_deprecated("basic_transactions"));
        
        let v2 = ProtocolEvolution::bitcoin_v2();
        assert_eq!(v2.version, 2);
        assert!(v2.has_feature("privacy_features"));
        assert!(v2.is_deprecated("legacy_addresses"));
    }
}
