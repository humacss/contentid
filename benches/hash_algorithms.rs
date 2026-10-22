use std::hint::black_box;

use contentid::{
    ContentHasher, LocalContentHasher, local_typed_content_id_u64, typed_content_id_u128,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};

#[derive(Clone, Default, Hash)]
struct LocalIdInput {
    record_id: u128,
    time: i64,
    source_id: u128,
    sequence: u64,
}

minbin::minbin_struct! { LocalIdInput [
    self.record_id: u128,
    self.time: i64,
    self.source_id: u128,
    self.sequence: u64
] }

fn input() -> LocalIdInput {
    LocalIdInput {
        record_id: 0x42c7_19ea_7638_d12b_911a_73ef_118c_94d1,
        time: 1_789_012_345,
        source_id: 0x17a3_9be1_20de_47c8_a836_bf05_5dc2_1179,
        sequence: 37,
    }
}

fn hash_algorithms(c: &mut Criterion) {
    let input = input();
    let encoded = minbin::to_bytes(&input).unwrap();
    let encoded_len = encoded.len() as u64;

    c.bench_function("minbin_local_id/serialize_only", |b| {
        b.iter(|| black_box(minbin::to_bytes(black_box(&input)).unwrap()));
    });

    let mut preencoded = c.benchmark_group("preencoded_local_id");
    preencoded.throughput(Throughput::Bytes(encoded_len));
    preencoded.bench_function("blake3_128", |b| {
        b.iter(|| {
            let mut hasher = ContentHasher::new();
            hasher.update_bytes(black_box(&encoded));
            black_box(hasher.finalize_u128())
        });
    });
    preencoded.bench_function("default_hasher_64", |b| {
        b.iter(|| {
            let mut hasher = LocalContentHasher::new();
            hasher.update_bytes(black_box(&encoded));
            black_box(hasher.finalize_u64())
        });
    });
    preencoded.finish();

    let mut group = c.benchmark_group("value_local_id");
    group.throughput(Throughput::Bytes(encoded_len));

    group.bench_function("blake3_128", |b| {
        b.iter(|| {
            let mut hasher = ContentHasher::new();
            hasher.update_value(black_box(&input)).unwrap();
            black_box(hasher.finalize_u128())
        });
    });

    group.bench_function("default_hasher_64", |b| {
        b.iter(|| {
            let mut hasher = LocalContentHasher::new();
            hasher.update_value(black_box(&input));
            black_box(hasher.finalize_u64())
        });
    });

    group.finish();

    let mut typed = c.benchmark_group("typed_local_id");
    typed.throughput(Throughput::Elements(1));
    typed.bench_function("blake3_128", |b| {
        b.iter(|| black_box(typed_content_id_u128("LocalIdInput", black_box(&input))));
    });
    typed.bench_function("default_hasher_64", |b| {
        b.iter(|| {
            black_box(local_typed_content_id_u64(
                "LocalIdInput",
                black_box(&input),
            ))
        });
    });
    typed.finish();
}

criterion_group!(benches, hash_algorithms);
criterion_main!(benches);
