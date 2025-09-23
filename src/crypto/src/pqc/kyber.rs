//! ML-KEM implementation based on CRYSTALS-Kyber

use crate::pqc::KeyEncapsulation;
use crate::Result;

// Concrete types - direct imports without unused trait aliases
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
        // Fix: encapsulate returns (SharedSecret, Ciphertext), we need (Ciphertext, SharedSecret)
        let (shared_secret, ciphertext) = encapsulate(public_key);
        Ok((ciphertext, shared_secret))
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
        
        // Convert to bytes for comparison
        let ss1_bytes = unsafe {
            std::slice::from_raw_parts(
                &ss1 as *const _ as *const u8,
                std::mem::size_of::<SharedSecret>(),
            )
        };
        let ss2_bytes = unsafe {
            std::slice::from_raw_parts(
                &ss2 as *const _ as *const u8,
                std::mem::size_of::<SharedSecret>(),
            )
        };
        
        assert_eq!(ss1_bytes, ss2_bytes);
    }
}
