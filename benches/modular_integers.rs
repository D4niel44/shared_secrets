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

macro_rules! ref_op_bench {
    ($op:tt) => {{
            let prime = Prime::parse("7").unwrap();
            let a = ModInteger::parse("3", &prime).unwrap();
            let b = ModInteger::parse("5", &prime).unwrap();
            black_box(&a $op &b);
    }}
}

fn ref_operations_benchmark(c: &mut Criterion) {
    c.bench_function("Ref Addition", |b| b.iter(|| ref_op_bench!(+)));
    c.bench_function("Ref Substraction", |b| b.iter(|| ref_op_bench!(-)));
    c.bench_function("Ref Multiplication", |b| b.iter(|| ref_op_bench!(*)));
    c.bench_function("Ref Division", |bench| bench.iter(|| ref_op_bench!(/)));
}

fn expensive_computation() {
    let prime = Prime::parse("5452212151844264453115521770362712815300456788073870260637172006414987479914150831821202259862091373741738511579629040732909194883").unwrap();
    let x = ModInteger::parse("54522121518442644531155217703627128153004567880738702606371720064149874799141508318212022598620913737417385115796290407329091948", &prime).unwrap();
    let x_1 = ModInteger::parse("545221215184426445311552177036271281530045678807387026063717200641498747991415083182120225986209137374173851157962904073290919471", &prime).unwrap();
    let x_2 = ModInteger::parse("445221215184426445311552177036271281530045678807387026063717200641498747991415083182120225986209137374173851157962904073290919471", &prime).unwrap();
    let x_3 = ModInteger::parse("445221215184486445311552177036271281530045678807387026063717200641498747991415083182120225986209137374173851157962904073290919471", &prime).unwrap();
    let x_4 = ModInteger::parse("445221215184486445311552677036271281530045678807387026063717200641498747991415083182120225986209137374173851157962904073290919471", &prime).unwrap();
    black_box(
        ((&x - &x_2) * (&x - &x_3) * (&x - &x_4)) / ((&x_1 - &x_2) * (&x_1 - &x_3) * (&x_1 - &x_4)),
    );
}

fn expensive_computation_benchmark(c: &mut Criterion) {
    c.bench_function("Expensive", |b| {
        b.iter(|| black_box(expensive_computation()))
    });
}

fn rng_benchmark(c: &mut Criterion) {
    let mut rng = Rng::new();
    let prime = Prime::parse("58021664585639791181184025950440248398226136069516938232493687505822471836536824298822733710342250697739996825938232641940670857624514103125986134050997697160127301547995788468137887651823707102007839").unwrap() ;
    c.bench_function("RNG", |b| {
        b.iter(|| black_box(ModInteger::random(black_box(&prime), &mut rng)))
    });
}

criterion_group!(
    benches,
    operations_benchmark,
    ref_operations_benchmark,
    expensive_computation_benchmark,
    rng_benchmark,
);
criterion_main!(benches);
