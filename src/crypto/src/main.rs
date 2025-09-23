// src/crypto/src/main.rs (Fixed formatting)
//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::pqc::kyber::{MlKem768, KeyEncapsulation};
use qraiop_crypto::{init, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ QRAIOP Quantum-Safe Cryptography Demo");
    
    // Initialize the library
    init()?;
    
    // Show library information
    let lib_info = info();
    println!("📚 Library Info: {:?}", lib_info);
    
    // Demonstrate ML-KEM-768
    println!("\n🔐 ML-KEM-768 Key Encapsulation Demo:");
    
    // Generate keypair
    println!("  📝 Generating keypair...");
    let (public_key, secret_key) = MlKem768::keypair()?;
    println!("  ✅ Keypair generated successfully");
    
    // Encapsulate shared secret
    println!("  🔒 Encapsulating shared secret...");
    let (ciphertext, shared_secret1) = MlKem768::encapsulate(&public_key)?;
    println!("  ✅ Shared secret encapsulated");
    
    // Decapsulate shared secret
    println!("  🔓 Decapsulating shared secret...");
    let shared_secret2 = MlKem768::decapsulate(&secret_key, &ciphertext)?;
    println!("  ✅ Shared secret decapsulated");
    
    // Verify shared secrets match
    if shared_secret1 == shared_secret2 {
        println!("  🎉 Shared secrets match! ML-KEM working correctly.");
    } else {
        println!("  ❌ Shared secrets don't match!");
        return Err("ML-KEM verification failed".into());
    }
    
    println!("\n✨ QRAIOP Quantum-Safe Cryptography Demo Complete!");
    
    Ok(())
}
