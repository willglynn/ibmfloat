use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use ibmfloat;
use rand::prelude::*;

fn f32_to_f32(c: &mut Criterion) {
    c.bench_function("C: F32 to f32", |b| {
        let mut rng = thread_rng();
        b.iter_batched(
            || rng.gen(),
            |input| {
                let foreign_float = ibmfloat::F32::from_bits(input);
                let native_float: f32 = foreign_float.into();
                black_box(native_float);
            },
            BatchSize::SmallInput,
        );
    });
}

fn f32_to_f64(c: &mut Criterion) {
    c.bench_function("C: F32 to f64", |b| {
        let mut rng = thread_rng();
        b.iter_batched(
            || rng.gen(),
            |input| {
                let foreign_float = ibmfloat::F32::from_bits(input);
                let native_float: f64 = foreign_float.into();
                black_box(native_float);
            },
            BatchSize::SmallInput,
        );
    });
}

fn f64_to_f32(c: &mut Criterion) {
    c.bench_function("C: F64 to f32", |b| {
        let mut rng = thread_rng();
        b.iter_batched(
            || rng.gen(),
            |input| {
                let foreign_float = ibmfloat::F64::from_bits(input);
                let native_float: f32 = foreign_float.into();
                black_box(native_float);
            },
            BatchSize::SmallInput,
        );
    });
}

fn f64_to_f64(c: &mut Criterion) {
    c.bench_function("C: F64 to f64", |b| {
        let mut rng = thread_rng();
        b.iter_batched(
            || rng.gen(),
            |input| {
                let foreign_float = ibmfloat::F64::from_bits(input);
                let native_float: f64 = foreign_float.into();
                black_box(native_float);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, f32_to_f32, f32_to_f64, f64_to_f32, f64_to_f64);
criterion_main!(benches);
