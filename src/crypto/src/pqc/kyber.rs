//! ML-KEM implementation based on CRYSTALS-Kyber

use crate::pqc::KeyEncapsulation;
use crate::{QraiopError, Result};

// Import trait for convenience methods
use pqcrypto_traits::kem::{PublicKey as PKTrait, SecretKey as SKTrait, Ciphertext as CiphTrait, SharedSecret as SSTrait};

// Concrete types
use pqcrypto_kyber::kyber512::{keypair, encapsulate, decapsulate, PublicKey, SecretKey, Ciphertext, SharedSecret};

pub struct MlKem512;

impl KeyEncapsulation for MlKem512 {
    type PublicKey = PublicKey;
    type SecretKey = SecretKey;
    type Ciphertext = Ciphertext;
    type SharedSecret = SharedSecret;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)> {
        Ok(keypair())
    }

    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)> {
        let pk = PKTrait::from_bytes(public_key.as_bytes())
            .map_err(|e| QraiopError::InvalidKey(e.to_string()))?;
        Ok(encapsulate(&pk))
    }

    fn decapsulate(
        secret_key: &Self::SecretKey,
        ciphertext: &Self::Ciphertext,
    ) -> Result<Self::SharedSecret> {
        let sk = SKTrait::from_bytes(secret_key.as_bytes())
            .map_err(|e| QraiopError::InvalidKey(e.to_string()))?;
        let ct = CiphTrait::from_bytes(ciphertext.as_bytes())
            .map_err(|e| QraiopError::EncapsulationFailed(e.to_string()))?;
        Ok(decapsulate(&ct, &sk))
    }
}
