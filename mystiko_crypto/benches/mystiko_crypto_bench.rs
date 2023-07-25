use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ff::PrimeField;
use mystiko_crypto::merkle_tree;
use num_bigint::BigUint;
use poseidon_rs::{Fr, Poseidon};

fn merkel_tree_new_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkel_tree_new_benchmark    ");
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();

    let elements = vec![e1.clone(); 32];
    group.bench_function("merkel_tree_32_element_benchmark", |b| {
        b.iter(|| merkle_tree::MerkleTree::new(black_box(Some(elements.clone())), Some(20), None).unwrap())
    });

    let elements = vec![e1; 1024];
    group.bench_function("merkel_tree_1024_element_benchmark", |b| {
        b.iter(|| merkle_tree::MerkleTree::new(black_box(Some(elements.clone())), Some(20), None).unwrap())
    });
}

fn poseidon_hash_benchmark(c: &mut Criterion) {
    let e1 = BigUint::parse_bytes(b"12d7aafbf3d4c1852ad3634d69607fc9ea8028f2d5724fcf3b917e71fd2dbff6", 16).unwrap();
    let fr1 = Fr::from_str(&e1.to_string()).unwrap();
    let frs = vec![fr1; 2];
    let poseidon = Poseidon::new();

    let mut group = c.benchmark_group("poseidon hash benchmark");
    group.bench_function("poseidon-rs", |b| {
        b.iter(|| poseidon.hash(black_box(frs.clone())).unwrap())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = merkel_tree_new_benchmark,poseidon_hash_benchmark
);
criterion_main!(benches);
