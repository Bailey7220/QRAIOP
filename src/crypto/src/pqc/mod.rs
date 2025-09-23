//! Post-Quantum Cryptography implementations

pub mod kyber;

pub trait KeyEncapsulation {
    type PublicKey;
    type SecretKey;
    type Ciphertext;
    type SharedSecret;

    fn keypair() -> crate::Result<(Self::PublicKey, Self::SecretKey)>;
    fn encapsulate(public_key: &Self::PublicKey)
        -> crate::Result<(Self::Ciphertext, Self::SharedSecret)>;
    fn decapsulate(
        secret_key: &Self::SecretKey,
        ciphertext: &Self::Ciphertext,
    ) -> crate::Result<Self::SharedSecret>;
}

pub trait DigitalSignature {
    // ...
}

pub trait HashBasedSignature: DigitalSignature {
    // ...
}
