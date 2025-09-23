// src/crypto/src/utils.rs
//! Utility functions for QRAIOP cryptography

use crate::{Result, QraiopError};
use std::fmt;

/// Convert bytes to hexadecimal string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert hexadecimal string to bytes
pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>> {
    hex::decode(hex_str).map_err(|e| QraiopError::SerializationError(e.to_string()))
}

/// Secure random number generation
pub fn secure_random(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Constant-time comparison
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    use subtle::ConstantTimeEq;
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

/// Benchmark helper
pub struct BenchmarkTimer {
    start: std::time::Instant,
}

impl BenchmarkTimer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
    
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
}

impl Default for BenchmarkTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Key size helpers
pub mod key_sizes {
    /// ML-KEM-512 key sizes
    pub mod ml_kem_512 {
        pub const PUBLIC_KEY_SIZE: usize = 800;
        pub const SECRET_KEY_SIZE: usize = 1632;
        pub const CIPHERTEXT_SIZE: usize = 768;
        pub const SHARED_SECRET_SIZE: usize = 32;
    }
    
    /// ML-KEM-768 key sizes
    pub mod ml_kem_768 {
        pub const PUBLIC_KEY_SIZE: usize = 1184;
        pub const SECRET_KEY_SIZE: usize = 2400;
        pub const CIPHERTEXT_SIZE: usize = 1088;
        pub const SHARED_SECRET_SIZE: usize = 32;
    }
    
    /// ML-KEM-1024 key sizes
    pub mod ml_kem_1024 {
        pub const PUBLIC_KEY_SIZE: usize = 1568;
        pub const SECRET_KEY_SIZE: usize = 3168;
        pub const CIPHERTEXT_SIZE: usize = 1568;
        pub const SHARED_SECRET_SIZE: usize = 32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let hex = bytes_to_hex(&data);
        assert_eq!(hex, "0123456789abcdef");
        
        let decoded = hex_to_bytes(&hex).unwrap();
        assert_eq!(decoded, data);
    }
    
    #[test]
    fn test_secure_random() {
        let data1 = secure_random(32);
        let data2 = secure_random(32);
        assert_eq!(data1.len(), 32);
        assert_eq!(data2.len(), 32);
        assert_ne!(data1, data2); // Should be different
    }
    
    #[test]
    fn test_constant_time_eq() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 2, 3, 5];
        
        assert!(constant_time_eq(&a, &b));
        assert!(!constant_time_eq(&a, &c));
        assert!(!constant_time_eq(&a, &[1, 2, 3])); // Different lengths
    }
    
    #[test]
    fn test_benchmark_timer() {
        let timer = BenchmarkTimer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 10.0);
    }
}
