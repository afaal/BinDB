use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bindb; 

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Standard test", |b| b.iter(|| todo!() ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);