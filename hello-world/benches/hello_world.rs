use criterion::{criterion_group, criterion_main, Criterion};

/// Benchmarking on the function [hello_world::hello].
fn bench_hello_world(c: &mut Criterion) {
    c.bench_function("bench_hello_world", |b| {
        b.iter(|| {
            // avoid compiler optimization though it does not look like there is a way to optimize.
            std::hint::black_box( 
                hello_world::hello()
            );
        })
    });
}

criterion_group!(
    benches,
    bench_hello_world,
);
criterion_main!(benches);