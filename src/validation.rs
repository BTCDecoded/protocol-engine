//! Protocol validation interface
//! 
//! This module provides protocol-specific validation that extends
//! the pure mathematical consensus rules with network-specific
//! and protocol-specific validation logic.

use crate::{BitcoinProtocolEngine, ProtocolVersion, NetworkParameters, Result};
use consensus_proof::{Block, Transaction, ValidationResult};
use consensus_proof::types::{OutPoint, UTXO};
use std::collections::HashMap;

/// Protocol-specific validation rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolValidationRules {
    /// Maximum block size for this protocol
    pub max_block_size: u32,
    /// Maximum transaction size for this protocol
    pub max_tx_size: u32,
    /// Maximum script size for this protocol
    pub max_script_size: u32,
    /// Whether SegWit is enabled
    pub segwit_enabled: bool,
    /// Whether Taproot is enabled
    pub taproot_enabled: bool,
    /// Whether RBF (Replace-By-Fee) is enabled
    pub rbf_enabled: bool,
    /// Minimum transaction fee rate
    pub min_fee_rate: u64,
    /// Maximum transaction fee rate
    pub max_fee_rate: u64,
}

impl ProtocolValidationRules {
    /// Get validation rules for a specific protocol version
    pub fn for_protocol(version: ProtocolVersion) -> Self {
        match version {
            ProtocolVersion::BitcoinV1 => Self::mainnet(),
            ProtocolVersion::Testnet3 => Self::testnet(),
            ProtocolVersion::Regtest => Self::regtest(),
        }
    }
    
    /// Mainnet validation rules (strict production rules)
    pub fn mainnet() -> Self {
        Self {
            max_block_size: 4_000_000, // 4MB block size limit
            max_tx_size: 1_000_000,    // 1MB transaction size limit
            max_script_size: 10_000,    // 10KB script size limit
            segwit_enabled: true,
            taproot_enabled: true,
            rbf_enabled: true,
            min_fee_rate: 1,           // 1 sat/vB minimum
            max_fee_rate: 1_000_000,   // 1M sat/vB maximum
        }
    }
    
    /// Testnet validation rules (same as mainnet but with testnet parameters)
    pub fn testnet() -> Self {
        Self {
            max_block_size: 4_000_000,
            max_tx_size: 1_000_000,
            max_script_size: 10_000,
            segwit_enabled: true,
            taproot_enabled: true,
            rbf_enabled: true,
            min_fee_rate: 1,
            max_fee_rate: 1_000_000,
        }
    }
    
    /// Regtest validation rules (relaxed for testing)
    pub fn regtest() -> Self {
        Self {
            max_block_size: 4_000_000,
            max_tx_size: 1_000_000,
            max_script_size: 10_000,
            segwit_enabled: true,
            taproot_enabled: true,
            rbf_enabled: true,
            min_fee_rate: 0,           // No minimum fee for testing
            max_fee_rate: 1_000_000,
        }
    }
    
}

/// Protocol-specific validation context
#[derive(Debug, Clone)]
pub struct ProtocolValidationContext {
    /// Current block height
    pub block_height: u64,
    /// Current network parameters
    pub network_params: NetworkParameters,
    /// Protocol validation rules
    pub validation_rules: ProtocolValidationRules,
    /// Additional context data
    pub context_data: HashMap<String, String>,
}

impl ProtocolValidationContext {
    /// Create validation context for a protocol version
    pub fn new(version: ProtocolVersion, block_height: u64) -> Result<Self> {
        let network_params = NetworkParameters::for_version(version)?;
        let validation_rules = ProtocolValidationRules::for_protocol(version);
        
        Ok(Self {
            block_height,
            network_params,
            validation_rules,
            context_data: HashMap::new(),
        })
    }
    
    /// Check if a feature is enabled at current block height
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        match feature {
            "segwit" => self.validation_rules.segwit_enabled,
            "taproot" => self.validation_rules.taproot_enabled,
            "rbf" => self.validation_rules.rbf_enabled,
            _ => false,
        }
    }
    
    /// Get maximum allowed size for a component
    pub fn get_max_size(&self, component: &str) -> u32 {
        match component {
            "block" => self.validation_rules.max_block_size,
            "transaction" => self.validation_rules.max_tx_size,
            "script" => self.validation_rules.max_script_size,
            _ => 0,
        }
    }
}

impl BitcoinProtocolEngine {
    /// Validate a block with protocol-specific rules
    pub fn validate_block_with_protocol(&self, block: &Block, utxos: &HashMap<OutPoint, UTXO>, height: u64, context: &ProtocolValidationContext) -> Result<ValidationResult> {
        // First, run consensus validation
        let (consensus_result, _) = self.consensus.validate_block(block, utxos.clone(), height)?;
        
        // Then, apply protocol-specific validation
        self.apply_protocol_validation(block, context)?;
        
        Ok(consensus_result)
    }
    
    /// Validate a transaction with protocol-specific rules
    pub fn validate_transaction_with_protocol(&self, tx: &Transaction, context: &ProtocolValidationContext) -> Result<ValidationResult> {
        // First, run consensus validation
        let consensus_result = self.consensus.validate_transaction(tx)?;
        
        // Then, apply protocol-specific validation
        self.apply_transaction_protocol_validation(tx, context)?;
        
        Ok(consensus_result)
    }
    
    /// Apply protocol-specific validation rules
    fn apply_protocol_validation(&self, block: &Block, context: &ProtocolValidationContext) -> Result<()> {
        // Check block size limits
        let block_size = self.calculate_block_size(block);
        if block_size > context.validation_rules.max_block_size {
            return Err(consensus_proof::error::ConsensusError::BlockValidation("Block size exceeds maximum".to_string()));
        }
        
        // Check transaction count limits
        if block.transactions.len() > 10000 { // Reasonable limit
            return Err(consensus_proof::error::ConsensusError::BlockValidation("Too many transactions in block".to_string()));
        }
        
        // Validate each transaction with protocol rules
        for tx in &block.transactions {
            self.apply_transaction_protocol_validation(tx, context)?;
        }
        
        Ok(())
    }
    
    /// Apply protocol-specific transaction validation
    fn apply_transaction_protocol_validation(&self, tx: &Transaction, context: &ProtocolValidationContext) -> Result<()> {
        // Check transaction size limits
        let tx_size = self.calculate_transaction_size(tx);
        if tx_size > context.validation_rules.max_tx_size {
            return Err(consensus_proof::error::ConsensusError::TransactionValidation("Transaction size exceeds maximum".to_string()));
        }
        
        // Check script size limits
        for input in &tx.inputs {
            if input.script_sig.len() > context.validation_rules.max_script_size as usize {
                return Err(consensus_proof::error::ConsensusError::TransactionValidation("Script size exceeds maximum".to_string()));
            }
        }
        
        for output in &tx.outputs {
            if output.script_pubkey.len() > context.validation_rules.max_script_size as usize {
                return Err(consensus_proof::error::ConsensusError::TransactionValidation("Script size exceeds maximum".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Calculate block size in bytes
    fn calculate_block_size(&self, block: &Block) -> u32 {
        // Simplified size calculation
        // In reality, this would include proper serialization
        let header_size = 80; // Block header is always 80 bytes
        let tx_count_size = 4; // Varint for transaction count
        let tx_sizes: u32 = block.transactions.iter()
            .map(|tx| self.calculate_transaction_size(tx))
            .sum();
        
        header_size + tx_count_size + tx_sizes
    }
    
    /// Calculate transaction size in bytes
    fn calculate_transaction_size(&self, tx: &Transaction) -> u32 {
        // Simplified size calculation
        let version_size = 4;
        let input_count_size = 4;
        let output_count_size = 4;
        let locktime_size = 4;
        
        let input_sizes: u32 = tx.inputs.iter()
            .map(|input| {
                32 + // prevout hash
                4 +  // prevout index
                input.script_sig.len() as u32 +
                4   // sequence
            })
            .sum();
        
        let output_sizes: u32 = tx.outputs.iter()
            .map(|output| {
                8 + // value
                output.script_pubkey.len() as u32
            })
            .sum();
        
        version_size + input_count_size + input_sizes + output_count_size + output_sizes + locktime_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation_rules() {
        let mainnet_rules = ProtocolValidationRules::mainnet();
        assert_eq!(mainnet_rules.max_block_size, 4_000_000);
        assert!(mainnet_rules.segwit_enabled);
        assert!(mainnet_rules.taproot_enabled);
        
        let regtest_rules = ProtocolValidationRules::regtest();
        assert_eq!(regtest_rules.max_block_size, 4_000_000);
        assert!(regtest_rules.segwit_enabled);
        assert_eq!(regtest_rules.min_fee_rate, 0); // No minimum fee for testing
    }
    
    #[test]
    fn test_validation_context() {
        let context = ProtocolValidationContext::new(ProtocolVersion::BitcoinV1, 1000).unwrap();
        assert_eq!(context.block_height, 1000);
        assert!(context.is_feature_enabled("segwit"));
        assert!(!context.is_feature_enabled("nonexistent"));
        assert_eq!(context.get_max_size("block"), 4_000_000);
    }
}
