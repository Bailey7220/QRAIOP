//! QRAIOP Quantum-Safe Cryptography Library

use oqs::kem::{Kem, Algorithm as KemAlg, PublicKey as KemPublicKey, SecretKey as KemSecretKey};
use oqs::sig::{Sig, Algorithm as SigAlg, Signature, PublicKey as SigPublicKey, SecretKey as SigSecretKey};

/// KEM keypair
pub struct KemKeypair {
    pub public_key: KemPublicKey,
    pub secret_key: KemSecretKey,
}

/// Generate a Kyber768 keypair
pub fn generate_kyber768_keypair() -> Result<KemKeypair, oqs::Error> {
    let kem = Kem::new(KemAlg::Kyber768)?;
    let (pk, sk) = kem.keypair()?;
    Ok(KemKeypair { public_key: pk, secret_key: sk })
}

/// Signature keypair
pub struct SigKeypair {
    pub public_key: SigPublicKey,
    pub secret_key: SigSecretKey,
}

/// Generate a Dilithium2 keypair
pub fn generate_dilithium2_keypair() -> Result<SigKeypair, oqs::Error> {
    let engine = Sig::new(SigAlg::Dilithium2)?;
    let (pk, sk) = engine.keypair()?;
    Ok(SigKeypair { public_key: pk, secret_key: sk })
}

/// Sign a message with Dilithium2
pub fn sign_message(sk: &SigKeypair, message: &[u8]) -> Result<Signature, oqs::Error> {
    let engine = Sig::new(SigAlg::Dilithium2)?;
    engine.sign(message, &sk.secret_key)
}

/// Verify a Dilithium2 signature
pub fn verify_signature(pk: &SigKeypair, message: &[u8], signature: &Signature) -> bool {
    let engine = Sig::new(SigAlg::Dilithium2).unwrap();
    engine.verify(message, signature, &pk.public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber768_keypair() {
        let kem = Kem::new(KemAlg::Kyber768).unwrap();
        let kp = generate_kyber768_keypair().expect("KEM keypair failed");
        assert_eq!(kp.public_key.as_ref().len(), kem.length_public_key());
        assert_eq!(kp.secret_key.as_ref().len(), kem.length_secret_key());
    }

    #[test]
    fn test_dilithium2_sign_verify() {
        let kp = generate_dilithium2_keypair().expect("Signature keypair failed");
        let msg = b"QRAIOP signing test";
        let signature = sign_message(&kp, msg).expect("Signing failed");
        assert!(verify_signature(&kp, msg, &signature));
    }
}
