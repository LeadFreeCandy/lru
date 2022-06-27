use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru::*;




fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cache test simple", |b| b.iter(|| {
        let mut cache = simple_lru::Cache::new(black_box(5000));
        for i in 0..10000 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 10000..0 {
            cache.set(black_box(i), black_box(i));
        }

        // for i in 0..10000 {
        //     black_box(cache.get(&i));
        // }

    }));

    c.bench_function("cache test fast", |b| b.iter(|| {
        let mut cache = fast_lru::Cache::new(black_box(5000));
        for i in 0..10000 {
            cache.set(black_box(i), black_box(i));
        }

        for i in 10000..0 {
            cache.set(black_box(i), black_box(i));
        }

        // for i in 0..10000 {
        //     black_box(cache.get(&i));
        // }

    }));

    c.bench_function("cache test unsafe", |b| b.iter(|| {
        let mut cache = lru_lib::LruCache::new(black_box(5000));
        for i in 0..10000 {
            cache.put(black_box(i), black_box(i));
        }

        for i in 10000..0 {
            cache.put(black_box(i), black_box(i));
        }

        // for i in 0..10000 {
        //     black_box(cache.get(&i));
        // }

    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);