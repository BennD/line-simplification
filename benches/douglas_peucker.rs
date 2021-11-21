use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use line_simplification::douglas_peucker::douglas_peucker;
use line_simplification::types::Point;

fn generate_sin_curve(start: f32, end: f32, y_multi: f32, n_points: u32) -> Vec<Point> {
    let diff = end - start;
    (0..n_points)
        .map(|i| {
            let x = start + (diff * (i as f32 / (n_points - 1) as f32));
            let y = x.sin() * y_multi;
            Point { x, y }
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sin_curve");
    for n_points in (1..=15).map(|i| i * 500_000) {
        let points = generate_sin_curve(0.0, 100.0, 10.0, n_points);

        group.throughput(Throughput::Elements(n_points as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n_points), &points, |b, points| {
            b.iter(|| douglas_peucker(black_box(points), black_box(1.0)))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
