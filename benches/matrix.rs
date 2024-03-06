use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

use matrix::Matrix;

fn multiplication_benchmark(c: &mut Criterion) {
    c.bench_function("matrix-multiplication", |b| {
        let mut rng = rand::thread_rng();

        let ref mut matrix_iterator = (0..)
            .map(|_| Matrix::<f64, 4, 4>::new([
                [rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64),],
                [rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64),],
                [rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64),],
                [rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64), rng.gen_range(-10.0..10.0f64),],
            ]));

        b.iter(|| { for _ in 0..1000 {
            let a = matrix_iterator.next().unwrap();
            let b = matrix_iterator.next().unwrap();

            black_box(a * b);
        }});
    });
}

criterion_group!(benches, multiplication_benchmark);
criterion_main!(benches);
