use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bin_pack; 

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Standard test", |b| b.iter(|| bin_pack::hello()) );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);