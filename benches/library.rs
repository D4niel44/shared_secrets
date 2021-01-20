use criterion::{criterion_group, criterion_main, Criterion};

use shared_secrets;
use shared_secrets::{Config, DecryptConfig, EncryptConfig};

fn benchmarks(c: &mut Criterion) {
    c.bench_function("Encrypt and Decrypt", |b| {
        b.iter(|| {
            let encrypt_config = EncryptConfig {
                total_evals: 5,
                min_required_evals: 4,
                input_file: "test_data/msg1.txt".into(),
                output_file: "ciphered".into(),
                password: "secure password".into(),
            };
            shared_secrets::run(Config::Encrypt(encrypt_config)).unwrap();
            let decrypt_config = DecryptConfig {
                shares_file: "ciphered.frg".into(),
                encrypted_file: "ciphered.aes".into(),
            };
            shared_secrets::run(Config::Decrypt(decrypt_config)).unwrap();
            std::fs::remove_file("ciphered.aes").unwrap();
            std::fs::remove_file("ciphered.frg").unwrap();
            std::fs::remove_file("msg1.txt").unwrap();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
