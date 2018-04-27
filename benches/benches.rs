#[macro_use]
extern crate criterion;
extern crate wu_diff;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("100 equal items", |b| {
        let slice = [0u8; 100];
        b.iter(|| ::wu_diff::diff(&slice, &slice));
    });

    c.bench_function("100 non-equal items", |b| {
        let (left, right) = ([0u8; 100], [1u8; 100]);
        b.iter(|| ::wu_diff::diff(&left, &right));
    });

    c.bench_function("200 non equal items in 1000", |d| {
        let left: Vec<u8> = vec![0; 1000]
            .iter()
            .enumerate()
            .map(|(i, _)| if i % 4 == 0 { 1 } else { 0 })
            .collect();
        let right: Vec<u8> = vec![0; 1000];
        d.iter(|| ::wu_diff::diff(&left, &right));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
