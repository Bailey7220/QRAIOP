//! ML-KEM implementation based on CRYSTALS-Kyber

use crate::pqc::KeyEncapsulation;
use crate::Result;

// Concrete types - direct imports
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
        
        // Compare shared secrets using constant-time comparison
        use std::ptr;
        let ss1_ptr = &ss1 as *const SharedSecret as *const u8;
        let ss2_ptr = &ss2 as *const SharedSecret as *const u8;
        let len = std::mem::size_of::<SharedSecret>();
        
        let equal = unsafe {
            libc::memcmp(ss1_ptr as *const libc::c_void, ss2_ptr as *const libc::c_void, len) == 0
        };
        
        assert!(equal, "Shared secrets should match");
    }
}
