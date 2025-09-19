//! QRAIOP Quantum-Safe Cryptography Library

use oqs::kem::Kem;
use oqs::kem::Algorithm;

pub struct KemKeypair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
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
        let kp = generate_kyber768_keypair().expect("Keypair generation failed");
        assert_eq!(kp.public_key.len(), Kem::new(Algorithm::Kyber768).unwrap().length_public_key());
        assert_eq!(kp.secret_key.len(), Kem::new(Algorithm::Kyber768).unwrap().length_secret_key());
    }
}

