use criterion::{criterion_group, criterion_main, Criterion};
use wql_nom::parse_wql;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_entity", |b| {
        b.iter(|| parse_wql("create entity my_entity"))
    });

    c.bench_function("inser_entity", |b| {
        b.iter(|| parse_wql("insert {a: 1, b: 2.3, c: 'g', d: \"str\",} into my_entity"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
