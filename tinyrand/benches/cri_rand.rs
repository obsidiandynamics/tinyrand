use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, RngCore, thread_rng};
use tinyrand::{Probability, Rand, RandRange, Wyrand, Xorshift};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rand = Xorshift::default();
    c.bench_function("xorshift/next_u64", |b| {
        b.iter(|| rand.next_u64());
    });
    c.bench_function("xorshift/next_u128", |b| {
        b.iter(|| rand.next_u128());
    });
    c.bench_function("xorshift/next_range<u64>", |b| {
        b.iter(|| rand.next_range(0..17u64));
    });
    c.bench_function("xorshift/next_range<u128>/small", |b| {
        b.iter(|| rand.next_range(0..17u128));
    });
    c.bench_function("xorshift/next_range<u128>/large", |b| {
        b.iter(|| rand.next_range(0..1u128 << 80));
    });
    let p = Probability::new(0.5);
    c.bench_function("xorshift/next_bool", |b| {
        b.iter(|| rand.next_bool(p));
    });

    let mut rand = Wyrand::default();
    c.bench_function("wyrand/next_u64", |b| {
        b.iter(|| rand.next_u64());
    });
    c.bench_function("wyrand/next_u128", |b| {
        b.iter(|| rand.next_u128());
    });
    c.bench_function("wyrand/next_range<u64>", |b| {
        b.iter(|| rand.next_range(0..17u64));
    });
    c.bench_function("wyrand/next_range<u128>/small", |b| {
        b.iter(|| rand.next_range(0..17u128));
    });
    c.bench_function("wyrand/next_range<u128>/large", |b| {
        b.iter(|| rand.next_range(0..1u128 << 80));
    });
    let p = Probability::new(0.5);
    c.bench_function("wyrand/next_bool", |b| {
        b.iter(|| rand.next_bool(p));
    });

    let mut rand = thread_rng();
    c.bench_function("rand/next_u64", |b| {
        b.iter(|| rand.next_u64());
    });
    c.bench_function("rand/next_range<u64>", |b| {
        b.iter(|| rand.gen_range(0..17u64));
    });
    c.bench_function("rand/next_range<u128>/small", |b| {
        b.iter(|| rand.gen_range(0..17u128));
    });
    c.bench_function("rand/next_range<u128>/large", |b| {
        b.iter(|| rand.gen_range(0..1u128 << 80));
    });
    c.bench_function("rand/next_bool", |b| {
        b.iter(|| rand.gen_bool(0.5));
    });

    let rand = fastrand::Rng::default();
    c.bench_function("fastrand/next_u64", |b| {
        b.iter(|| rand.u64(0..u64::MAX));
    });
    c.bench_function("fastrand/next_u128", |b| {
        b.iter(|| rand.u128(0..u128::MAX));
    });
    c.bench_function("fastrand/next_range<u64>", |b| {
        b.iter(|| rand.u64(0..17));
    });
    c.bench_function("fastrand/next_range<u128>/small", |b| {
        b.iter(|| rand.u128(0..17u128));
    });
    c.bench_function("fastrand/next_range<u128>/large", |b| {
        b.iter(|| rand.u128(0..1u128 << 80));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
