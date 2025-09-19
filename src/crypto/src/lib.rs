//! QRAIOP Quantum-Safe Cryptography Library

use oqs::kem::{Kem, Algorithm, PublicKey, SecretKey};

pub struct KemKeypair {
    pub public_key: oqs::kem::PublicKey,
    pub secret_key: oqs::kem::SecretKey,
}

pub fn generate_kyber768_keypair() -> Result<KemKeypair, oqs::Error> {
    let kem = Kem::new(Algorithm::Kyber768)?;
    let (pk, sk) = kem.keypair()?;
    Ok(KemKeypair { public_key: pk, secret_key: sk })
}

use oqs::sig::{Sig, Algorithm as SigAlg};

// Structure for signature keypair
pub struct SigKeypair {
    pub public_key: SigAlg::PublicKey,
    pub secret_key: SigAlg::SecretKey,
}

/// Generate Dilithium2 keypair
pub fn generate_dilithium2_keypair() -> Result<SigKeypair, oqs::Error> {
    let sig = Sig::new(SigAlg::Dilithium2)?;
    let (pk, sk) = sig.keypair()?;
    Ok(SigKeypair { public_key: pk, secret_key: sk })
}

/// Sign a message
pub fn sign_message(
    sk: &SigKeypair,
    message: &[u8],
) -> Result<Vec<u8>, oqs::Error> {
    let mut sig_engine = Sig::new(SigAlg::Dilithium2)?;
    let signature = sig_engine.sign(message, &sk.secret_key)?;
    Ok(signature)
}

/// Verify a signature
pub fn verify_signature(
    pk: &SigKeypair,
    message: &[u8],
    signature: &[u8],
) -> bool {
    let mut sig_engine = Sig::new(SigAlg::Dilithium2).unwrap();
    sig_engine.verify(message, signature, &pk.public_key).is_ok()
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

#[cfg(test)]
mod sig_tests {
    use super::*;

    #[test]
    fn test_dilithium2_sign_verify() {
        let kp = generate_dilithium2_keypair().expect("Keypair gen failed");
        let msg = b"QRAIOP signing test";
        let signature = sign_message(&kp, msg).expect("Signing failed");
        assert!(verify_signature(&kp, msg, &signature));
        // Tamper and fail
        let mut bad = signature.clone();
        bad[0] ^= 0xFF;
        assert!(!verify_signature(&kp, msg, &bad));
    }
}
