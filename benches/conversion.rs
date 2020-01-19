use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use ibmfloat;
use rand::prelude::*;

fn f32_to_f32(c: &mut Criterion) {
    c.bench_function("F32 to f32", |b| {
        let mut rng = thread_rng();
        b.iter_batched(
            || rng.gen(),
            |input: u32| {
                let foreign_float = ibmfloat::F32::from_bits(input);
                let native_float: f32 = foreign_float.into();
                black_box(native_float);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, f32_to_f32);
criterion_main!(benches);
