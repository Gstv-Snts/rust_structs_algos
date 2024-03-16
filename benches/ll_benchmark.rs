use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_structs_algos::ll::Queue;

pub fn queue_enqueue(c: &mut Criterion) {
    c.bench_function("queue_enqueue", |b| {
        b.iter(|| Queue::new().enqueue(black_box(10)))
    });
}

pub fn queue_dequeue(c: &mut Criterion) {
    c.bench_function("queue_dequeue", |b| {
        b.iter(|| {
            let mut d = Queue::new();
            d.enqueue(black_box(10));
            d.dequeue();
        })
    });
}

pub fn queue_is_empty(c: &mut Criterion) {
    c.bench_function("queue_im_empty", |b| {
        b.iter(|| {
            let mut d = Queue::new();
            d.enqueue(black_box(10));
            d.dequeue();
        })
    });
}

criterion_group!(benches, queue_enqueue, queue_dequeue, queue_is_empty);
criterion_main!(benches);
