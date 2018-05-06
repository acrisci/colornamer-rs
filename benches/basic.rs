#[macro_use]
extern crate criterion;
extern crate colornamer;

use criterion::Criterion;
use colornamer::{name_color_hex,Colors};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic", |b| b.iter(|| name_color_hex("#1E90FF", Colors::HTML)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
