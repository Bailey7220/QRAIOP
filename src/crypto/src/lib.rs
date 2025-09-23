//! QRAIOP Quantum-Resistant Cryptographic Library
//!
//! This library provides production-ready implementations of NIST-approved
//! post-quantum cryptographic algorithms including ML-KEM, ML-DSA, and SLH-DSA.

use std::fmt;
use zeroize::Zeroize;

pub mod pqc;
pub mod hybrid;
pub mod utils;

// Re-export main types
pub use pqc::{DigitalSignature, HashBasedSignature, KeyEncapsulation};
pub use hybrid::{HybridKem, HybridSignature};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Supported algorithms
pub const ALGORITHMS: &[&str] = &[
    "ML-KEM-512",
    "ML-KEM-768",
    "ML-KEM-1024",
    "ML-DSA-44",
    "ML-DSA-65",
    "ML-DSA-87",
    "SLH-DSA-128s",
    "SLH-DSA-192s",
    "SLH-DSA-256s",
];

/// Main error type for the library
#[derive(Debug, thiserror::Error)]
pub enum QraiopError {
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Algorithm not supported: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Key encapsulation failed: {0}")]
    EncapsulationFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for library operations
pub type Result<T> = std::result::Result<T, QraiopError>;

/// Security level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Level1 = 1,
    Level3 = 3,
    Level5 = 5,
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityLevel::Level1 => write!(f, "Level-1"),
            SecurityLevel::Level3 => write!(f, "Level-3"),
            SecurityLevel::Level5 => write!(f, "Level-5"),
        }
    }
}

/// Initialize the library with logging
pub fn init() -> Result<()> {
    env_logger::init();
    log::info!("QRAIOP Crypto Library v{} initialized", VERSION);
    log::info!("Supported algorithms: {:?}", ALGORITHMS);
    Ok(())
}

/// Get library information
pub fn info() -> LibraryInfo {
    LibraryInfo {
        version: VERSION.to_string(),
        supported_algorithms: ALGORITHMS.iter().map(|&s| s.to_string()).collect(),
        nist_approved: true,
        quantum_resistant: true,
    }
}

/// Library information structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryInfo {
    pub version: String,
    pub supported_algorithms: Vec<String>,
    pub nist_approved: bool,
    pub quantum_resistant: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_init() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_library_info() {
        let info = info();
        assert_eq!(info.version, VERSION);
        assert!(info.nist_approved);
        assert!(info.quantum_resistant);
        assert!(!info.supported_algorithms.is_empty());
    }
}
