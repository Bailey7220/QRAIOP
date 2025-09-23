//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::pqc::kyber::{KeyEncapsulation, MlKem768};
use qraiop_crypto::{init, info};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🛡️ QRAIOP Quantum-Safe Cryptography Demo");

    init()?;
    let lib_info = info();
    println!("📚 Library Info: {:?}", lib_info);

    println!("\n🔐 ML-KEM-768 Key Encapsulation Demo:");
    println!("  📝 Generating keypair...");
    let (public_key, secret_key) = MlKem768::keypair()?;
    println!("  ✅ Keypair generated successfully");

    println!("  🔒 Encapsulating shared secret...");
    let (ciphertext, shared_secret1) = MlKem768::encapsulate(&public_key)?;
    println!("  ✅ Shared secret encapsulated");

    println!("  🔓 Decapsulating shared secret...");
    let shared_secret2 = MlKem768::decapsulate(&secret_key, &ciphertext)?;
    println!("  ✅ Shared secret decapsulated");

    if shared_secret1 == shared_secret2 {
        println!("  🎉 Shared secrets match! ML-KEM working correctly.");
    } else {
        println!("  ❌ Shared secrets don't match!");
        return Err("ML-KEM verification failed".into());
    }

    println!("\n✨ QRAIOP Quantum-Safe Cryptography Demo Complete!");
    Ok(())
}
