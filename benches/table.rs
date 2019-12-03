use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use ffxiv_rrc::calc;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("whole_data_set");
    group.throughput(Throughput::Bytes(16));
    for players in 1..=24 {
        group.bench_with_input(
            BenchmarkId::from_parameter(players),
            &players,
            |b, &players| {
                b.iter(|| calc(black_box(players), black_box(90)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
