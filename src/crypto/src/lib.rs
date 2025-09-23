//! QRAIOP Quantum-Resistant Cryptographic Library
//!
//! This library provides production-ready implementations of NIST-approved
//! post-quantum cryptographic algorithms including ML-KEM, ML-DSA, and SLH-DSA.

pub mod pqc;
pub mod utils;

// Re-export main types
pub use pqc::{DigitalSignature, HashBasedSignature, KeyEncapsulation};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

pub type Result<T> = std::result::Result<T, QraiopError>;

pub fn init() -> Result<()> {
    env_logger::init();
    Ok(())
}

pub fn info() -> LibraryInfo {
    LibraryInfo {
        version: VERSION.to_string(),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryInfo {
    pub version: String,
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
    }
}
