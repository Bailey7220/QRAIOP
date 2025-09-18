//! QRAIOP Quantum-Safe Cryptography Library
//! 
//! Provides post-quantum cryptographic algorithms for quantum-resilient infrastructure.

/// Check if an algorithm is quantum-safe
pub fn is_quantum_safe(algorithm: &str) -> bool {
    matches!(algorithm,
        "ML-KEM-512" | "ML-KEM-768" | "ML-KEM-1024" |
        "ML-DSA-44" | "ML-DSA-65" | "ML-DSA-87"
    )
}

/// Get list of supported quantum-safe algorithms
pub fn supported_algorithms() -> Vec<&'static str> {
    vec![
        "ML-KEM-512", "ML-KEM-768", "ML-KEM-1024",
        "ML-DSA-44", "ML-DSA-65", "ML-DSA-87"
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_safe_detection() {
        assert!(is_quantum_safe("ML-KEM-768"));
        assert!(!is_quantum_safe("RSA-2048"));
    }

    #[test]
    fn test_supported_algorithms() {
        let algorithms = supported_algorithms();
        assert!(!algorithms.is_empty());
        assert!(algorithms.contains(&"ML-KEM-768"));
    }
}
