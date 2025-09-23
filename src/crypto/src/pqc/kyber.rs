//! ML-KEM (Module-Lattice-based Key Encapsulation Mechanism) implementation
//!
//! Based on CRYSTALS-Kyber, standardized as FIPS 203.

use crate::pqc::KeyEncapsulation;
use crate::{QraiopError, Result, SecurityLevel};

// Import the trait that provides as_bytes() and from_bytes()
use pqcrypto_traits::kem::PublicKey as PQPublicKeyTrait;
use pqcrypto_traits::kem::SecretKey as PQSecretKeyTrait;
use pqcrypto_traits::kem::Ciphertext as PQCiphertextTrait;
use pqcrypto_traits::kem::SharedSecret as PQSharedSecretTrait;

// Import the concrete types
use pqcrypto_kyber::{PublicKey, SecretKey, Ciphertext, SharedSecret};
use pqcrypto_kyber::{kyber512, kyber768, kyber1024};

/// ML-KEM-512 implementation (Security Level 1)
pub struct MlKem512;
pub struct MlKem768;
pub struct MlKem1024;

impl KeyEncapsulation for MlKem512 {
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Ciphertext = Ciphertext;
    type SharedSecret = SharedSecret;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)> {
        let (pk, sk) = kyber512::keypair();
        Ok((pk, sk))
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        // from_bytes is available via the PublicKey trait
        let pk = PQPublicKeyTrait::from_bytes(public_key.as_bytes())
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-512 public key: {}", e)))?;
        let (ct, ss) = kyber512::encapsulate(&pk);
        Ok((ct, ss))
    }

    fn decapsulate(
        secret_key: &Self::SecretKey,
        ciphertext: &Self::Ciphertext,
    ) -> Result<Self::SharedSecret> {
        let sk = PQSecretKeyTrait::from_bytes(secret_key.as_bytes())
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-512 secret key: {}", e)))?;
        let ct = PQCiphertextTrait::from_bytes(ciphertext.as_bytes())
            .map_err(|e| QraiopError::EncapsulationFailed(format!("Invalid ciphertext: {}", e)))?;
        let ss = kyber512::decapsulate(&ct, &sk);
        Ok(ss)
    }

    fn security_level() -> SecurityLevel {
        SecurityLevel::Level1
    }

    fn algorithm_name() -> &'static str {
        "ML-KEM-512"
    }
}

// Repeat for MlKem768 and MlKem1024 similarly...
