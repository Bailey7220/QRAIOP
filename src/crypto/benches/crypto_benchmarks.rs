// src/crypto/benches/crypto_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qraiop_crypto::pqc::kyber::{MlKem512, MlKem768, MlKem1024, KeyEncapsulation};

fn benchmark_ml_kem_512(c: &mut Criterion) {
    c.bench_function("ML-KEM-512 keypair", |b| {
        b.iter(|| {
            let _result = black_box(MlKem512::keypair().unwrap());
        })
    });

    let (pk, sk) = MlKem512::keypair().unwrap();
    
    c.bench_function("ML-KEM-512 encapsulate", |b| {
        b.iter(|| {
            let _result = black_box(MlKem512::encapsulate(&pk).unwrap());
        })
    });

    let (ct, _ss) = MlKem512::encapsulate(&pk).unwrap();
    
    c.bench_function("ML-KEM-512 decapsulate", |b| {
        b.iter(|| {
            let _result = black_box(MlKem512::decapsulate(&sk, &ct).unwrap());
        })
    });
}

fn benchmark_ml_kem_768(c: &mut Criterion) {
    c.bench_function("ML-KEM-768 keypair", |b| {
        b.iter(|| {
            let _result = black_box(MlKem768::keypair().unwrap());
        })
    });
}

fn benchmark_ml_kem_1024(c: &mut Criterion) {
    c.bench_function("ML-KEM-1024 keypair", |b| {
        b.iter(|| {
            let _result = black_box(MlKem1024::keypair().unwrap());
        })
    });
}

criterion_group!(benches, benchmark_ml_kem_512, benchmark_ml_kem_768, benchmark_ml_kem_1024);
criterion_main!(benches);
