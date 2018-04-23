#[macro_use]
extern crate criterion;
extern crate wu_diff;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("10 equal items", |b| {
        let slice = [0u8; 100];
        b.iter(|| ::wu_diff::diff(&slice, &slice));
    });

    c.bench_function("100 non-equal items", |b| {
        let (left, right) = ([0u8; 1000], [1u8; 1000]);
        b.iter(|| ::wu_diff::diff(&left, &right));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
