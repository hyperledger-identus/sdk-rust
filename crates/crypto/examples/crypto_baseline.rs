#![forbid(unsafe_code)]

//! Measurement-only cryptography baseline for the weekly/manual slow lane.

use std::env;
use std::hint::black_box;
use std::time::Instant;

use identus_crypto::path::DerivationAxis;
use identus_crypto::{
    CardanoV2ExtendedPrivateKey, Ed25519PrivateKey, HDKey, MnemonicHelper, P256PrivateKey,
    Secp256k1PrivateKey, Verifiable, X25519PrivateKey, sha256, sha512,
};
use serde::Serialize;

const MIN_SAMPLES: usize = 20;
const WARMUP_BATCHES: usize = 1;
const MESSAGE: &[u8] = b"identus-sdk-rust crypto performance baseline";

#[derive(Serialize)]
struct Artifact {
    schema_version: u32,
    status: &'static str,
    apollo_comparison: &'static str,
    revision: String,
    compiler: String,
    os: &'static str,
    arch: &'static str,
    cpu: String,
    environment: String,
    feature_profile: &'static str,
    samples: usize,
    warmup_batches: usize,
    operations: Vec<Measurement>,
}

#[derive(Serialize)]
struct Measurement {
    name: &'static str,
    iterations_per_sample: usize,
    p50_ns: u128,
    p95_ns: u128,
    min_ns: u128,
    max_ns: u128,
}

fn required_metadata(name: &str) -> Result<String, String> {
    let value =
        env::var(name).map_err(|_| format!("missing required environment variable {name}"))?;
    let value = value.trim();
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(format!("invalid metadata in {name}"));
    }
    Ok(value.to_owned())
}

fn parse_samples<I>(arguments: I) -> Result<usize, String>
where
    I: IntoIterator<Item = String>,
{
    let mut arguments = arguments.into_iter();
    let mut samples = MIN_SAMPLES;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--samples" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--samples requires a value".to_owned())?;
                samples = value
                    .parse::<usize>()
                    .map_err(|_| "--samples must be an integer".to_owned())?;
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }
    if samples < MIN_SAMPLES {
        return Err(format!("--samples must be at least {MIN_SAMPLES}"));
    }
    Ok(samples)
}

fn nearest_rank(sorted: &[u128], percentile: usize) -> u128 {
    assert!(!sorted.is_empty(), "percentile requires a sample");
    let rank = (percentile * sorted.len()).div_ceil(100);
    sorted[rank.saturating_sub(1)]
}

fn measure<T, F>(
    name: &'static str,
    iterations_per_sample: usize,
    samples: usize,
    mut operation: F,
) -> Measurement
where
    F: FnMut() -> T,
{
    for _ in 0..iterations_per_sample {
        black_box(operation());
    }

    let mut observed = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        for _ in 0..iterations_per_sample {
            black_box(operation());
        }
        observed.push(started.elapsed().as_nanos() / iterations_per_sample as u128);
    }
    observed.sort_unstable();

    Measurement {
        name,
        iterations_per_sample,
        p50_ns: nearest_rank(&observed, 50),
        p95_ns: nearest_rank(&observed, 95),
        min_ns: observed[0],
        max_ns: observed[observed.len() - 1],
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let samples = parse_samples(env::args().skip(1))?;
    let revision = required_metadata("IDENTUS_BENCH_REVISION")?;
    let compiler = required_metadata("IDENTUS_BENCH_COMPILER")?;
    let cpu = required_metadata("IDENTUS_BENCH_CPU")?;
    let environment = required_metadata("IDENTUS_BENCH_ENVIRONMENT")?;

    let ed_private = Ed25519PrivateKey::from_slice(&[1; 32])?;
    let ed_public = ed_private.to_public_key();
    let ed_signature = ed_private.sign(MESSAGE);

    let secp_private = Secp256k1PrivateKey::from_slice(&[2; 32])?;
    let secp_public = secp_private.to_public_key();
    let secp_signature = secp_private.sign(MESSAGE);

    let p256_private = P256PrivateKey::from_slice(&[3; 32])?;
    let p256_public = p256_private.to_public_key();
    let p256_signature = p256_private.sign(MESSAGE);

    let x_private = X25519PrivateKey::from_slice(&[4; 32])?;
    let peer_public = X25519PrivateKey::from_slice(&[5; 32])?.to_public_key();

    let mnemonic = MnemonicHelper::to_mnemonic_code(&[0; 16]);
    let hd_parent = HDKey::init_from_seed(&[6; 32])?;
    let hardened_zero = DerivationAxis::hardened(0);

    let cardano_parent = CardanoV2ExtendedPrivateKey::from_nonextended(&[7; 32], &[8; 32]);
    let cardano_public = cardano_parent.to_public_key()?;
    let soft_zero = DerivationAxis::normal(0);

    let operations = vec![
        measure("sha-256", 10_000, samples, || sha256(black_box(MESSAGE))),
        measure("sha-512", 10_000, samples, || sha512(black_box(MESSAGE))),
        measure("ed25519-sign", 500, samples, || {
            ed_private.sign(black_box(MESSAGE))
        }),
        measure("ed25519-verify", 200, samples, || {
            ed_public.verify(black_box(MESSAGE), black_box(&ed_signature))
        }),
        measure("x25519-agreement", 1_000, samples, || {
            x_private.derive_shared(black_box(&peer_public))
        }),
        measure("secp256k1-sign", 100, samples, || {
            secp_private.sign(black_box(MESSAGE))
        }),
        measure("secp256k1-verify", 100, samples, || {
            secp_public.verify(black_box(MESSAGE), black_box(&secp_signature))
        }),
        measure("p256-sign", 100, samples, || {
            p256_private.sign(black_box(MESSAGE))
        }),
        measure("p256-verify", 100, samples, || {
            p256_public.verify(black_box(MESSAGE), black_box(&p256_signature))
        }),
        measure("bip39-seed", 5, samples, || {
            MnemonicHelper::create_seed(black_box(&mnemonic), black_box("TREZOR"))
                .expect("fixed BIP-39 input remains valid")
        }),
        measure("secp256k1-bip32-hardened-child", 500, samples, || {
            hd_parent
                .derive_child(black_box(hardened_zero))
                .expect("fixed BIP-32 parent remains valid")
        }),
        measure("cardano-v2-private-child", 200, samples, || {
            cardano_parent
                .derive_child(black_box(soft_zero))
                .expect("fixed Cardano private parent remains valid")
        }),
        measure("cardano-v2-public-child", 200, samples, || {
            cardano_public
                .derive_child(black_box(soft_zero))
                .expect("fixed Cardano public parent remains valid")
        }),
    ];

    let artifact = Artifact {
        schema_version: 1,
        status: "measurement-only",
        apollo_comparison: "unavailable",
        revision,
        compiler,
        os: env::consts::OS,
        arch: env::consts::ARCH,
        cpu,
        environment,
        feature_profile: "default",
        samples,
        warmup_batches: WARMUP_BATCHES,
        operations,
    };
    println!("{}", serde_json::to_string_pretty(&artifact)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("crypto-baseline: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_rank_percentiles_are_deterministic() {
        let values = (1..=20).collect::<Vec<_>>();
        assert_eq!(nearest_rank(&values, 50), 10);
        assert_eq!(nearest_rank(&values, 95), 19);
    }

    #[test]
    fn sample_floor_is_enforced() {
        assert_eq!(parse_samples(Vec::<String>::new()).unwrap(), MIN_SAMPLES);
        assert!(parse_samples(["--samples".to_owned(), "19".to_owned()]).is_err());
        assert!(parse_samples(["--unknown".to_owned()]).is_err());
    }
}
