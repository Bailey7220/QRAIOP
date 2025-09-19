//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::{generate_kyber768_keypair};

fn main() {
    println!("🛡️ QRAIOP Quantum-Safe Cryptography Demo");
    println!("==========================================");

    // Generate a Kyber768 keypair
    match generate_kyber768_keypair() {
        Ok(kp) => {
            println!("✅ Generated Kyber768 keypair:");
            println!("  Public key length : {}", kp.public_key.as_ref().len());
            println!("  Secret key length : {}", kp.secret_key.as_ref().len());
        }
        Err(e) => {
            eprintln!("❌ Failed to generate keypair: {}", e);
        }
    }
}
