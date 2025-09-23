//! ML-KEM (Module-Lattice-based Key Encapsulation Mechanism) implementation
//!
//! Based on CRYSTALS-Kyber, standardized as FIPS 203.

use crate::pqc::KeyEncapsulation;
use crate::{QraiopError, Result, SecurityLevel};
use pqcrypto_kyber::*;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

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
        Ok((PublicKey(pk.as_bytes().to_vec()), SecretKey(sk.as_bytes().to_vec())))
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        let pk = kyber512::PublicKey::from_bytes(&public_key.0)
            .map_err(|e| QraiopError::InvalidKey(format!("ML-KEM-512 public key: {}", e)))?;
        let (ct, ss) = kyber512::encapsulate(&pk);
        Ok((Ciphertext(ct.as_bytes().to_vec()), SharedSecret(ss.as_bytes().to_vec())))
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

// Similarly for MlKem768 and MlKem1024...

