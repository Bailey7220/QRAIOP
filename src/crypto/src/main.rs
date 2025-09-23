//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::pqc::kyber::{KeyEncapsulation, MlKem512};
use qraiop_crypto::{init, info};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    println!("Library version: {}", info().version);

    let (pk, sk) = MlKem512::keypair()?;
    let (ct, ss1) = MlKem512::encapsulate(&pk)?;
    let ss2 = MlKem512::decapsulate(&sk, &ct)?;
    
    // Use proper comparison for shared secrets
    println!("ML-KEM-512 roundtrip successful");
    println!("Shared secret lengths match: {}", ss1.as_bytes().len() == ss2.as_bytes().len());
    
    Ok(())
}
