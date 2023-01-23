use criterion::{criterion_group, criterion_main, measurement::Measurement, BatchSize, BenchmarkGroup, Criterion, black_box};
use fuel_types::Bytes32;
use rkyv::{AlignedVec, to_bytes};
use array_concat::{CustomKey, MultiKey, NewMultiKey};



criterion_group!(benches, multikey);
criterion_main!(benches);


fn multikey(c: &mut Criterion) {
    let k1 = Bytes32::new([1u8; 32]);
    let k2 = Bytes32::new([2u8; 32]);


    c.bench_function("multikey", |b| b.iter(|| multikey_creation(black_box(&k1), black_box(&k2))));
    c.bench_function("new multikey", |b| b.iter(|| new_multikey_creation(black_box(&k1), black_box(&k2))));
    c.bench_function("custom key", |b| b.iter(|| custom_key(black_box(k1), black_box(k2))));
}


fn multikey_creation<'a>(a: &'a Bytes32, b: &'a Bytes32) -> MultiKey<&'a Bytes32, &'a Bytes32> {
    MultiKey::new(&(a, b))
}

fn new_multikey_creation(a: &Bytes32, b: &Bytes32) -> NewMultiKey {
    NewMultiKey::new(a, b)
}

fn custom_key(k1: Bytes32, k2: Bytes32) -> AlignedVec {
    let ck = CustomKey {
        k1: k1.into(),
        k2: k2.into(),
    };
    ck.to_bytes()
}

