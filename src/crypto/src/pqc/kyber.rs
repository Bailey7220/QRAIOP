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
        // Use the concrete type directly instead of trait conversion
        Ok(encapsulate(public_key))
    }

    fn decapsulate(
        secret_key: &Self::SecretKey,
        ciphertext: &Self::Ciphertext,
    ) -> Result<Self::SharedSecret> {
        // Use the concrete types directly
        Ok(decapsulate(ciphertext, secret_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_kem_512_roundtrip() {
        let (pk, sk) = MlKem512::keypair().unwrap();
        let (ct, ss1) = MlKem512::encapsulate(&pk).unwrap();
        let ss2 = MlKem512::decapsulate(&sk, &ct).unwrap();
        // Note: Direct equality comparison might not work, use secure comparison in production
        assert_eq!(ss1.as_bytes(), ss2.as_bytes());
    }
}
