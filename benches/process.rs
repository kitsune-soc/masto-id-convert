use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn process_bench(c: &mut Criterion) {
    c.bench_function("process 110368129515784116", |b| {
        b.iter(|| masto_id_convert::process(black_box("110368129515784116")))
    });
}

criterion_group!(benches, process_bench);
criterion_main!(benches);
