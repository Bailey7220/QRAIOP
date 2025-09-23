// src/crypto/src/pqc/kyber.rs
//! ML-KEM (Module-Lattice-based Key Encapsulation Mechanism) implementation
//! 
//! Based on CRYSTALS-Kyber, standardized as FIPS 203.

use crate::pqc::KeyEncapsulation;
use crate::{QraiopError, Result, SecurityLevel};
use pqcrypto_kyber::*;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// ML-KEM-512 implementation (Security Level 1)
pub struct MlKem512;

/// ML-KEM-768 implementation (Security Level 3)  
pub struct MlKem768;

/// ML-KEM-1024 implementation (Security Level 5)
pub struct MlKem1024;

/// Public key wrapper with serialization support
#[derive(Clone, Serialize, Deserialize)]
pub struct PublicKey(Vec<u8>);

/// Secret key wrapper with secure memory handling
#[derive(Clone, ZeroizeOnDrop)]
pub struct SecretKey(Vec<u8>);

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

/// Ciphertext wrapper with serialization support
#[derive(Clone, Serialize, Deserialize)]
pub struct Ciphertext(Vec<u8>);

/// Shared secret wrapper with secure memory handling
#[derive(Clone, ZeroizeOnDrop)]
pub struct SharedSecret(Vec<u8>);

impl Zeroize for SharedSecret {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl PartialEq for SharedSecret {
    fn eq(&self, other: &Self) -> bool {
        use subtle::ConstantTimeEq;
        self.0.ct_eq(&other.0).into()
    }
}

// ML-KEM-512 Implementation
impl KeyEncapsulation for MlKem512 {
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Ciphertext = Ciphertext;
    type SharedSecret = SharedSecret;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)> {
        let (pk, sk) = kyber512::keypair();
        Ok((
            PublicKey(pk.as_bytes().to_vec()),
            SecretKey(sk.as_bytes().to_vec()),
        ))
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        let pk = kyber512::PublicKey::from_bytes(&public_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-512 public key: {}", e)))?;
        
        let (ct, ss) = kyber512::encapsulate(&pk);
        
        Ok((
            Ciphertext(ct.as_bytes().to_vec()),
            SharedSecret(ss.as_bytes().to_vec()),
        ))
    }

    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Result<Self::SharedSecret> {
        let sk = kyber512::SecretKey::from_bytes(&secret_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-512 secret key: {}", e)))?;
        
        let ct = kyber512::Ciphertext::from_bytes(&ciphertext.0)
            .map_err(|e| QraiopError::EncapsulationFailed(format!("Invalid ciphertext: {}", e)))?;
        
        let ss = kyber512::decapsulate(&ct, &sk);
        
        Ok(SharedSecret(ss.as_bytes().to_vec()))
    }

    fn security_level() -> SecurityLevel {
        SecurityLevel::Level1
    }

    fn algorithm_name() -> &'static str {
        "ML-KEM-512"
    }
}

// ML-KEM-768 Implementation
impl KeyEncapsulation for MlKem768 {
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Ciphertext = Ciphertext;
    type SharedSecret = SharedSecret;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)> {
        let (pk, sk) = kyber768::keypair();
        Ok((
            PublicKey(pk.as_bytes().to_vec()),
            SecretKey(sk.as_bytes().to_vec()),
        ))
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        let pk = kyber768::PublicKey::from_bytes(&public_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-768 public key: {}", e)))?;
        
        let (ct, ss) = kyber768::encapsulate(&pk);
        
        Ok((
            Ciphertext(ct.as_bytes().to_vec()),
            SharedSecret(ss.as_bytes().to_vec()),
        ))
    }

    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Result<Self::SharedSecret> {
        let sk = kyber768::SecretKey::from_bytes(&secret_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-768 secret key: {}", e)))?;
        
        let ct = kyber768::Ciphertext::from_bytes(&ciphertext.0)
            .map_err(|e| QraiopError::EncapsulationFailed(format!("Invalid ciphertext: {}", e)))?;
        
        let ss = kyber768::decapsulate(&ct, &sk);
        
        Ok(SharedSecret(ss.as_bytes().to_vec()))
    }

    fn security_level() -> SecurityLevel {
        SecurityLevel::Level3
    }

    fn algorithm_name() -> &'static str {
        "ML-KEM-768"
    }
}

// ML-KEM-1024 Implementation  
impl KeyEncapsulation for MlKem1024 {
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Ciphertext = Ciphertext;
    type SharedSecret = SharedSecret;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)> {
        let (pk, sk) = kyber1024::keypair();
        Ok((
            PublicKey(pk.as_bytes().to_vec()),
            SecretKey(sk.as_bytes().to_vec()),
        ))
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        let pk = kyber1024::PublicKey::from_bytes(&public_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-1024 public key: {}", e)))?;
        
        let (ct, ss) = kyber1024::encapsulate(&pk);
        
        Ok((
            Ciphertext(ct.as_bytes().to_vec()),
            SharedSecret(ss.as_bytes().to_vec()),
        ))
    }

    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Result<Self::SharedSecret> {
        let sk = kyber1024::SecretKey::from_bytes(&secret_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-1024 secret key: {}", e)))?;
        
        let ct = kyber1024::Ciphertext::from_bytes(&ciphertext.0)
            .map_err(|e| QraiopError::EncapsulationFailed(format!("Invalid ciphertext: {}", e)))?;
        
        let ss = kyber1024::decapsulate(&ct, &sk);
        
        Ok(SharedSecret(ss.as_bytes().to_vec()))
    }

    fn security_level() -> SecurityLevel {
        SecurityLevel::Level5
    }

    fn algorithm_name() -> &'static str {
        "ML-KEM-1024"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_kem_512() {
        let (pk, sk) = MlKem512::keypair().unwrap();
        let (ct, ss1) = MlKem512::encapsulate(&pk).unwrap();
        let ss2 = MlKem512::decapsulate(&sk, &ct).unwrap();
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn test_ml_kem_768() {
        let (pk, sk) = MlKem768::keypair().unwrap();
        let (ct, ss1) = MlKem768::encapsulate(&pk).unwrap();
        let ss2 = MlKem768::decapsulate(&sk, &ct).unwrap();
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn test_ml_kem_1024() {
        let (pk, sk) = MlKem1024::keypair().unwrap();
        let (ct, ss1) = MlKem1024::encapsulate(&pk).unwrap();
        let ss2 = MlKem1024::decapsulate(&sk, &ct).unwrap();
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn test_security_levels() {
        assert_eq!(MlKem512::security_level(), SecurityLevel::Level1);
        assert_eq!(MlKem768::security_level(), SecurityLevel::Level3);
        assert_eq!(MlKem1024::security_level(), SecurityLevel::Level5);
    }

    #[test]
    fn test_algorithm_names() {
        assert_eq!(MlKem512::algorithm_name(), "ML-KEM-512");
        assert_eq!(MlKem768::algorithm_name(), "ML-KEM-768");
        assert_eq!(MlKem1024::algorithm_name(), "ML-KEM-1024");
    }
}

