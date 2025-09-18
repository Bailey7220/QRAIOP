//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::{is_quantum_safe, supported_algorithms};

fn main() {
    println!("🛡️ QRAIOP Quantum-Safe Cryptography Demo");
    println!("==========================================");
    
    println!("\n📋 Supported Algorithms:");
    for algorithm in supported_algorithms() {
        let status = if is_quantum_safe(algorithm) { "✅" } else { "❌" };
        println!("  {} {}", status, algorithm);
    }
    
    println!("\n🎉 Demo complete!");
}
