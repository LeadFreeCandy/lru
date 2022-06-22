use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru::*;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cache set", |b| b.iter(|| {
        let mut cache = Cache::new(black_box(500));
        for i in 0..10000 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 10000..0 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 0..10000 {
            black_box(cache.get(&i));
        }

    }));

    c.bench_function("cache set 2", |b| b.iter(|| {
        let mut cache = Cache::new(black_box(500));
        for i in 0..10000 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 10000..0 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 0..10000 {
            black_box(cache.get(&i));
        }

    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);