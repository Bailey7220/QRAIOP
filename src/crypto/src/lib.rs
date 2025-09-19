//! QRAIOP Quantum-Safe Cryptography Library

use oqs::kem::{Kem, Algorithm, PublicKey, SecretKey};
use oqs::kem::Algorithm;

pub struct KemKeypair {
    pub public_key: oqs::kem::PublicKey,
    pub secret_key: oqs::kem::SecretKey,
}

pub fn generate_kyber768_keypair() -> Result<KemKeypair, oqs::Error> {
    let kem = Kem::new(Algorithm::Kyber768)?;
    let (pk, sk) = kem.keypair()?;
    Ok(KemKeypair { public_key: pk, secret_key: sk })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber768_keypair() {
        let kem = Kem::new(Algorithm::Kyber768).unwrap();
        let kp = generate_kyber768_keypair().unwrap();
        assert_eq!(kp.public_key.as_ref().len(), kem.length_public_key());
        assert_eq!(kp.secret_key.as_ref().len(), kem.length_secret_key());

    }
}

