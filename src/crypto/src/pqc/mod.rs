//! Post-Quantum Cryptography implementations
//!
//! This module provides implementations of NIST-standardized post-quantum
//! cryptographic algorithms.

pub mod kyber;
pub mod dilithium;
pub mod sphincs;

use crate::{QraiopError, Result, SecurityLevel};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub trait KeyEncapsulation {
    type PublicKey: Clone + Serialize + for<'de> Deserialize<'de>;
    type SecretKey: Clone + Zeroize + ZeroizeOnDrop;
    type Ciphertext: Clone + Serialize + for<'de> Deserialize<'de>;
    type SharedSecret: Clone + Zeroize + ZeroizeOnDrop;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)>;
    fn encapsulate(public_key: &Self::PublicKey) -> Result<(Self::Ciphertext, Self::SharedSecret)>;
    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Result<Self::SharedSecret>;
    fn security_level() -> SecurityLevel;
    fn algorithm_name() -> &'static str;
}

pub trait DigitalSignature {
    type PublicKey: Clone + Serialize + for<'de> Deserialize<'de>;
    type SecretKey: Clone + Zeroize + ZeroizeOnDrop;
    type Signature: Clone + Serialize + for<'de> Deserialize<'de>;

    fn keypair() -> Result<(Self::PublicKey, Self::SecretKey)>;
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> Result<Self::Signature>;
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> Result<bool>;
    fn security_level() -> SecurityLevel;
    fn algorithm_name() -> &'static str;
}

pub trait HashBasedSignature: DigitalSignature {
    fn signatures_remaining(secret_key: &Self::SecretKey) -> Result<u64>;
    fn max_signatures() -> u64;
}

/// Performance metrics struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub algorithm: String,
    pub security_level: String,
    pub keypair_time_ms: f64,
    pub sign_time_ms: f64,
    pub verify_time_ms: f64,
    pub encap_time_ms: f64,
    pub decap_time_ms: f64,
    pub public_key_size: usize,
    pub secret_key_size: usize,
    pub signature_size: usize,
    pub ciphertext_size: usize,
}

/// Benchmark KEM algorithms
pub fn benchmark_kem<K: KeyEncapsulation>() -> Result<PerformanceMetrics> {
    use std::time::Instant;

    let start = Instant::now();
    let (pk, sk) = K::keypair()?;
    let keypair_time = start.elapsed().as_secs_f64() * 1000.0;

    let start = Instant::now();
    let (ct, _ss1) = K::encapsulate(&pk)?;
    let encap_time = start.elapsed().as_secs_f64() * 1000.0;

    let start = Instant::now();
    let _ss2 = K::decapsulate(&sk, &ct)?;
    let decap_time = start.elapsed().as_secs_f64() * 1000.0;

    Ok(PerformanceMetrics {
        algorithm: K::algorithm_name().to_string(),
        security_level: K::security_level().to_string(),
        keypair_time_ms: keypair_time,
        sign_time_ms: 0.0,
        verify_time_ms: 0.0,
        encap_time_ms: encap_time,
        decap_time_ms: decap_time,
        public_key_size: bincode::serialize(&pk).unwrap().len(),
        secret_key_size: std::mem::size_of_val(&sk),
        signature_size: 0,
        ciphertext_size: bincode::serialize(&ct).unwrap().len(),
    })
}

/// Benchmark signature algorithms
pub fn benchmark_signature<S: DigitalSignature>() -> Result<PerformanceMetrics> {
    use std::time::Instant;

    let message = b"QRAIOP quantum-resistant test message";

    let start = Instant::now();
    let (pk, sk) = S::keypair()?;
    let keypair_time = start.elapsed().as_secs_f64() * 1000.0;

    let start = Instant::now();
    let sig = S::sign(&sk, message)?;
    let sign_time = start.elapsed().as_secs_f64() * 1000.0;

    let start = Instant::now();
    let _valid = S::verify(&pk, message, &sig)?;
    let verify_time = start.elapsed().as_secs_f64() * 1000.0;

    Ok(PerformanceMetrics {
        algorithm: S::algorithm_name().to_string(),
        security_level: S::security_level().to_string(),
        keypair_time_ms: keypair_time,
        sign_time_ms: sign_time,
        verify_time_ms: verify_time,
        encap_time_ms: 0.0,
        decap_time_ms: 0.0,
        public_key_size: bincode::serialize(&pk).unwrap().len(),
        secret_key_size: std::mem::size_of_val(&sk),
        signature_size: bincode::serialize(&sig).unwrap().len(),
        ciphertext_size: 0,
    })
}

