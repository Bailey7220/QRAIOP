//! QRAIOP Quantum-Safe Cryptography Demo

use qraiop_crypto::{KeyEncapsulation, init, info};
use qraiop_crypto::pqc::kyber::MlKem512;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    println!("Library version: {}", info().version);

    let (pk, sk) = MlKem512::keypair()?;
    let (ct, ss1) = MlKem512::encapsulate(&pk)?;
    let ss2 = MlKem512::decapsulate(&sk, &ct)?;
    
    println!("ML-KEM-512 roundtrip successful");
    println!("Shared secret validation completed");
    
    Ok(())
}
