use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rust_structs_algos::{
    algos::{
        search::{async_linear_search, binary_search, linear_search},
        sort::{bubble_sort, quick_sort},
    },
    rayon_par_iter, regular_iter,
};

pub fn iter(c: &mut Criterion) {
    const SIZE: usize = 10000;
    let mut rng = rand::thread_rng();
    let v: Vec<u32> = (0..SIZE).map(|_| rng.gen_range(0..100)).collect();
    c.bench_function("iter", |b| b.iter(|| regular_iter(&v)));
    c.bench_function("par_iter", |b| b.iter(|| rayon_par_iter(&v)));
}
pub fn search(c: &mut Criterion) {
    let mut v = black_box(vec![0; 10000]);
    for i in 0..10000 {
        v[i] = rand::thread_rng().gen_range(0..1000);
    }
    c.bench_function("binary_search", |b| b.iter(|| binary_search(&v, 25)));
    c.bench_function("linear_search", |b| b.iter(|| linear_search(&v, 25)));
    c.bench_function("async_linear_search", |b| {
        b.iter(|| async_linear_search(&v, 25))
    });
}
pub fn sort(c: &mut Criterion) {
    let mut v = black_box([0; 10000]);
    for i in 0..10000 {
        v[i] = rand::thread_rng().gen_range(0..1000);
    }
    c.bench_function("bubble_sort", |b| b.iter(|| bubble_sort(&mut v)));
    c.bench_function("quick_sort", |b| b.iter(|| quick_sort(&mut v)));
}

criterion_group!(benches, search, sort, iter);
criterion_main!(benches);
