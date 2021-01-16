use criterion::{black_box, criterion_group, criterion_main, Criterion};

use shared_secrets::math::{random::Rng, ModInteger, Prime};

macro_rules! op_bench {
    ($op:tt) => {{
            let prime = Prime::parse("7").unwrap();
            let a = ModInteger::parse("3", &prime).unwrap();
            let b = ModInteger::parse("5", &prime).unwrap();
            black_box(a $op b);
    }}
}
fn operations_benchmark(c: &mut Criterion) {
    c.bench_function("Addition", |bench| bench.iter(|| op_bench!(+)));
    c.bench_function("Substraction", |bench| bench.iter(|| op_bench!(-)));
    c.bench_function("Multiplication", |bench| bench.iter(|| op_bench!(*)));
    c.bench_function("Division", |bench| bench.iter(|| op_bench!(/)));
}

criterion_group!(benches, operations_benchmark);
criterion_main!(benches);
