use criterion::{BenchmarkId, black_box, Criterion, criterion_group, criterion_main};
use hexx::*;

pub fn line_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hex line");
    group.significance_level(0.1).sample_size(1_000);
    let dist = 100_000;

    group.bench_with_input(BenchmarkId::new("Line", dist), &dist, |b, dist| {
        b.iter(|| {
            let p = black_box(Hex::ZERO);
            p.line_to(black_box(Hex::splat(*dist))).collect::<Vec<_>>()
        })
    });
    group.finish();
}

criterion_group!(benches, line_benchmark);
criterion_main!(benches);
